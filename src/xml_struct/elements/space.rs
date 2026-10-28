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

pub struct Space {}

impl ElementBase for Space {
    fn new(_: Box<XmlElement>, _: &mut ElementMappings, _: i32) -> Result<Self, ElementError> {
        return Ok(Self {});
    }

    fn render<'a>(
        &self,
        _: &'a ElementsManager,
        datas: ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut space = iced::widget::Space::new();

        space = space.width(theme.width).height(theme.height);

        return space.into();
    }
}
