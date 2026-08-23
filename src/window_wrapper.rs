use std::any::Any;

use crate::{
    app_manager::App,
    dom::{
        events::DomQueryResult,
        query::{QueryBuilder, QueryResponse},
    },
    xml_engine::{Message, XmlEngine},
    xml_struct::{elements::library::generate_element_from_tag, parser::XmlElement},
};

pub type AppResult = iced::Result;

pub fn render<'a>(engine: &'a XmlEngine) -> iced::Element<'a, Message> {
    return engine.view();
}
#[cfg(feature = "css-watcher")]
pub type CssChannelType = notify::Result<notify::Event>;
#[cfg(feature = "css-watcher")]
pub type CssRx = std::sync::mpsc::Receiver<CssChannelType>;
#[cfg(feature = "css-watcher")]
pub type CssTx = std::sync::mpsc::Sender<CssChannelType>;
#[cfg(feature = "css-watcher")]
pub type CssWatcher = notify::RecommendedWatcher;

pub struct Objects<'a, WindowApp, AppState> {
    pub engine: &'a mut XmlEngine,
    pub qb: &'a mut QueryBuilder<WindowApp, AppState>,
    #[cfg(feature = "css-watcher")]
    pub css_watcher_rx: Option<&'a mut CssRx>,
    #[cfg(feature = "css-watcher")]
    pub css_path: &'a str,
}

pub struct ObjectsReadOnly<'a, WindowApp, AppState> {
    pub engine: &'a XmlEngine,
    pub qb: &'a QueryBuilder<WindowApp, AppState>,
}

pub trait WindowTemplate<WindowApp: 'static, AppState: 'static> {
    fn get_objects(&mut self) -> Objects<'_, WindowApp, AppState>;
    fn get_objects_read_only(&self) -> ObjectsReadOnly<'_, WindowApp, AppState>;
    fn get_self(&mut self) -> &mut WindowApp;
    fn update(&mut self, message: crate::xml_engine::Message, app_state: &mut App<AppState>) {
        let me = self.get_objects();
        for (_, component) in me.engine.window.element_renderer.components.iter_mut() {
            (me.engine.window.element_renderer.functions.update)(
                component,
                message.clone(),
                app_state,
            );
        }
        let responses = me.engine.update(message);
        for (callback, response) in me.qb.fetch(responses) {
            let me = self.get_self();
            callback(me, response, app_state);
        }
    }
    fn add_component(
        &mut self,
        element: DomQueryResult,
        component: Box<dyn Any>,
    ) -> Result<i32, &str> {
        let me = self.get_objects();
        let uid = me
            .engine
            .window
            .element_renderer
            .raw_element_query(&element.get_query().query_type);
        if uid.len() != 1 {
            return Err(
                "Panic: More than one element found when adding a component. Please ensure that the query is unique.",
            );
        }
        me.engine
            .window
            .element_renderer
            .register_component(uid[0], component);
        return Ok(uid[0]);
    }
    fn remove_component(&mut self, component_uid: i32, app_state: &mut App<AppState>) {
        let me = self.get_objects();
        let elem_renderer = &mut me.engine.window.element_renderer;
        let component = elem_renderer.components.remove(&component_uid);
        if component.is_none() {
            panic!("Component with id {} not found", component_uid);
        }
        (elem_renderer.functions.on_close)(&mut component.unwrap(), app_state);
        let element = generate_element_from_tag(&XmlElement::void(), elem_renderer, component_uid);
        if element.is_some() {
            elem_renderer.init_element(
                element.unwrap(),
                Some(XmlElement::void()),
                None,
                component_uid,
            );
        } else {
            panic!("Block: <Void /> doesn't exists");
        }
    }
    fn process(&mut self) -> QueryResponse {
        let me = self.get_objects();
        for query in me.qb.execute(me.engine) {
            let me = self.get_self();
            (query.0)(me, query.1);
        }
        let me = self.get_objects();
        return me.qb.last.clone();
    }
    // use the "render" helper
    fn render(&self) -> iced::Element<'_, crate::xml_engine::Message> {
        let me = self.get_objects_read_only();
        return render(me.engine);
    }
    // Subscription logic (for set_timeout and set_interval)
    fn subscription(&self) -> Vec<iced::Subscription<Message>> {
        let engine = self.get_objects_read_only().engine;
        let query_builder = self.get_objects_read_only().qb;
        return query_builder.subscribe(engine);
    }
    // // Gets the path to the CSS file
    // #[cfg(feature = "css-watcher")]
    // fn get_css_path(&self) -> &str;
    #[cfg(feature = "css-watcher")]
    fn set_css_watcher_rx(&mut self, rx: CssRx);

    // Events
    // When the window is closed
    #[allow(unused_variables)]
    fn on_close(&mut self, app_state: &mut App<AppState>) {}

    // Util to convert the window template into a Box<dyn Any>
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}
