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

pub struct Void {}

impl ElementBase for Void {
    fn new(_: &XmlElement, _: &mut ElementRenderer, _: i32) -> Self {
        Self {}
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

    fn process_event(
        &mut self,
        _: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        return None;
    }
}
