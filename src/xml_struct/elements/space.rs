// Copy-paste template
use crate::{
    dom::query::QueryResponse,
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        elements::element_base::ElementBase,
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub struct Space {}

impl ElementBase for Space {
    fn new(_: &XmlElement, _: &mut ElementRenderer, _: i32) -> Self {
        Self {}
    }

    fn render<'a>(
        &self,
        _: &'a ElementRenderer,
        datas: &'a ElementExtraData,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut space = iced::widget::Space::new();

        space = space.width(theme.width).height(theme.height);

        return space.into();
    }

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        match event {
            _ => None,
        }
    }
}
