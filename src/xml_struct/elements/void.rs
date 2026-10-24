// Copy-paste template
use crate::{
    elements_manager::{
        elements_manager::ElementsManager,
        mappings::ElementMappings,
        utils::{ElementExtraData, EventListener},
    },
    xml_engine::Message,
    xml_struct::{
        elements::{element_base::ElementBase, library::ElementError},
        xml_element::XmlElement,
    },
};

pub struct Void {}

impl ElementBase for Void {
    fn new(_: Box<XmlElement>, _: &mut ElementMappings, _: i32) -> Result<Self, ElementError> {
        return Ok(Self {});
    }

    fn render<'a>(
        &self,
        _: &'a ElementsManager,
        _: ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        return iced::widget::space().into();
    }
}
