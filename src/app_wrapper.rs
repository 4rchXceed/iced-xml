use iced::Element;

use crate::xml_engine::{Message, XmlEngine};

pub fn run_app<App>(
    new: fn() -> App,
    update: fn(&mut App, Message),
    view: fn(&App) -> Element<'_, Message>,
) -> iced::Result
where
    App: 'static,
    Message: Send + 'static,
{
    iced::application(new, update, view).run()
}

pub fn render(engine: &XmlEngine) -> iced::Element<'_, Message> {
    return engine.view();
}

pub trait AppTemplate {
    fn new() -> Self;
    // for query in self.qb.fetch(self.engine.update(message)) {
    //     (query.0)(self, query.1);
    // }
    fn update(&mut self, message: crate::xml_engine::Message);
    // for query in self.qb.execute(&mut self.engine) {
    //     (query.0)(self, query.1);
    // }
    fn process(&mut self);
    // use the "render" helper
    fn render(&self) -> iced::Element<'_, crate::xml_engine::Message>;
}
