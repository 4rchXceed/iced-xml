use crate::{Message, xml_struct::{elements::ElementRenderer, parser::XmlElement}};

pub trait ElementBase {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer) -> Self;
    fn render<'a>(&self, renderer: &'a ElementRenderer) -> iced::Element<'a, Message>;
}
