use crate::{
    dom::{
        events::{DomInternalMessageType, EventListenerTypes},
        query_builder::EventResponse,
    }, elements_manager::{elements_manager::ElementsManager, mappings::ElementMappings, utils::{ElementEventResponse, ElementExtraData, EventListener}}, xml_engine::Message, xml_struct::{
        elements::library::ElementError,
        parser::XmlElement,
    }
};

pub trait ElementBase
where
    Self: Sized,
{
    fn new(
        xml_element: Box<XmlElement>,
        mappings: &mut ElementMappings,
        self_uid: i32,
    ) -> Result<Self, ElementError>;
    fn render<'a>(
        &'a self,
        mappings: &'a ElementsManager,
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
        event_type: &EventListenerTypes,
        event_response: &EventResponse,
    ) -> Option<ElementEventResponse> {
        None
    }
}
