use crate::{
    app_manager::App,
    dom::query::{QueryBuilder, QueryResponse},
    xml_engine::{Message, XmlEngine},
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
        for (callback, response) in me.qb.fetch(me.engine.update(message)) {
            let me = self.get_self();
            callback(me, response, app_state);
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
    fn subscription(&self) -> iced::Subscription<Message> {
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
}
