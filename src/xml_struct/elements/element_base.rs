use crate::{
    dom::{events::DomInternalMessageType, query::EventResponse},
    xml_engine::Message,
    xml_struct::{
        element_renderer::{
            ElementEventResponse, ElementExtraData, ElementRenderer, EventListener,
        },
        parser::XmlElement,
    },
};

pub trait ElementBase {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer, self_uid: i32) -> Self;
    fn render<'a>(
        &'a self,
        renderer: &'a ElementRenderer,
        datas: ElementExtraData,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message>;
    // returns (query_response, elementsToForwardTheEvent)
    #[allow(unused_variables)]
    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        None
    }

    #[allow(unused_variables)]
    fn event_callback(
        &mut self,
        event_type: &String,
        event_response: &EventResponse,
    ) -> Option<ElementEventResponse> {
        None
    }
}
