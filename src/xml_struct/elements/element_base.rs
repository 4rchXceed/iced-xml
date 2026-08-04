use crate::{
    dom::query::QueryResponse,
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub trait ElementBase {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer, self_uid: i32) -> Self;
    fn render<'a>(
        &'a self,
        renderer: &'a ElementRenderer,
        datas: &'a ElementExtraData,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message>;
    // returns (query_response, elementsToForwardTheEvent)
    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)>;
}
