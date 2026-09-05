// Copy-paste template
use crate::{
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener},
        elements::{element_base::ElementBase, library::ElementError},
        parser::XmlElement,
    },
};

pub struct Space {}

impl ElementBase for Space {
    fn new(_: &XmlElement, _: &mut ElementRenderer, _: i32) -> Result<Self, ElementError> {
        return Ok(Self {});
    }

    fn render<'a>(
        &self,
        _: &'a ElementRenderer,
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
