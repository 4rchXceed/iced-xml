// Copy-paste template
use crate::{
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener},
        elements::{element_base::ElementBase, library::ElementError},
        parser::XmlElement,
    },
};

pub struct Void {}

impl ElementBase for Void {
    fn new(_: &XmlElement, _: &mut ElementRenderer, _: i32) -> Result<Self, ElementError> {
        return Ok(Self {});
    }

    fn render<'a>(
        &self,
        _: &'a ElementRenderer,
        _: ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        return iced::widget::space().into();
    }
}
