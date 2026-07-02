use crate::{
    dom::query::QueryBuilder,
    xml_engine::{Message, XmlEngine},
};

pub type AppResult = iced::Result;

pub fn run_app<App: AppTemplate<App> + 'static>() -> iced::Result {
    iced::application(
        move || {
            let mut app = App::new();
            app.post_construct();
            return app;
        },
        App::update,
        App::render,
    )
    .subscription(App::subscription)
    .run()
}

pub fn render(engine: &XmlEngine) -> iced::Element<'_, Message> {
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

pub struct Objects<'a, App> {
    pub engine: &'a mut XmlEngine,
    pub qb: &'a mut QueryBuilder<App>,
    #[cfg(feature = "css-watcher")]
    pub css_watcher_rx: Option<&'a mut CssRx>,
    #[cfg(feature = "css-watcher")]
    pub css_path: &'a str,
}

pub struct ObjectsReadOnly<'a, App> {
    pub engine: &'a XmlEngine,
    pub qb: &'a QueryBuilder<App>,
}

pub trait AppTemplate<T: 'static> {
    fn get_objects(&mut self) -> Objects<'_, T>;
    fn get_objects_read_only(&self) -> ObjectsReadOnly<'_, T>;
    fn get_self(&mut self) -> &mut T;

    fn new() -> Self;
    // for query in self.qb.fetch(self.engine.update(message)) {
    //     (query.0)(self, query.1);
    // }
    fn update(&mut self, message: crate::xml_engine::Message) {
        let me = self.get_objects();
        for query in me.qb.fetch(me.engine.update(message)) {
            let me = self.get_self();
            (query.0)(me, query.1);
        }
    }
    // for query in self.qb.execute(&mut self.engine) {
    //     (query.0)(self, query.1);
    // }
    fn process(&mut self) {
        let me = self.get_objects();
        for query in me.qb.execute(me.engine) {
            let me = self.get_self();
            (query.0)(me, query.1);
        }
    }
    // use the "render" helper
    fn render(&self) -> iced::Element<'_, crate::xml_engine::Message> {
        let me = self.get_objects_read_only();
        return render(me.engine);
    }
    // Subscription logic (for set_timeout and set_interval)
    fn subscription(&self) -> iced::Subscription<Message> {
        let engine = self.get_objects_read_only().engine;
        return self.get_objects_read_only().qb.subscribe(engine);
    }
    // Post-construct called ONLY by the run_app function
    fn post_construct(&mut self);
    // // Gets the path to the CSS file
    // #[cfg(feature = "css-watcher")]
    // fn get_css_path(&self) -> &str;
    #[cfg(feature = "css-watcher")]
    fn set_css_watcher_rx(&mut self, rx: CssRx);
}
