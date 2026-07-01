use crate::{
    xml_engine::Message,
    xml_struct::{
        elements::{ElementRenderer, EventListener},
        parser::{XmlChangeEvent, XmlElement, XmlTheme},
    },
};

pub trait ElementBase {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer) -> Self;
    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        theme: &'a XmlTheme,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message>;
    fn process_event(&mut self, event: &XmlChangeEvent);
}
