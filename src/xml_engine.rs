use std::io::Cursor;

use crate::dom::events::{DomInternalMessageType, DomMessage};
use crate::dom::query::{EventResponse, QueryResponse};
use crate::xml_struct::parser::{XmlChangeEvent, XmlParser};
use crate::xml_struct::window::XmlWindow;
use quick_xml::Reader;

#[derive(Debug, Clone, Hash)]
pub enum Message {
    DomEvent(i32, EventResponse),
}

pub enum DynamicEvent {
    SetTimeout(i32),  // time in milliseconds
    SetInterval(i32), // time in milliseconds
}

pub struct XmlEngine {
    pub window: XmlWindow,
    pub dyn_events: Vec<(i32, DynamicEvent)>, // (Callback UID, DynamicEvent)
}

impl XmlEngine {
    pub fn new(xml: Vec<u8>) -> Self {
        let content: String = String::from_utf8(xml).expect("Failed to parse XML content as UTF-8");
        let reader = Reader::from_reader(Cursor::new(content.into_bytes()));
        let window_parser = XmlParser::new(&mut reader.clone());
        let window = XmlWindow::new(window_parser.root);

        Self {
            window: window,
            dyn_events: Vec::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Vec<(i32, EventResponse)> {
        self.window.fired_events.clear();
        match message {
            Message::DomEvent(event_uid, event_data) => {
                let mut is_dynamic = false;
                if event_data.next_timeout.is_some() {
                    is_dynamic = true;
                    self.dyn_events.push((
                        event_uid,
                        DynamicEvent::SetTimeout(event_data.next_timeout.unwrap() as i32),
                    ));
                }
                self.window.emit_event(event_uid, event_data, is_dynamic);
            }
        };
        return self.window.fired_events.clone();
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        return self.window.render();
    }

    pub fn client_events(&mut self, query: &DomMessage) -> QueryResponse {
        match &query.message {
            DomInternalMessageType::SubscribeDynamicEvent(dynamic_event) => {
                match dynamic_event {
                    DynamicEvent::SetInterval(time) => {
                        self.dyn_events
                            .push((query.uid, DynamicEvent::SetInterval(*time)));
                    }
                    DynamicEvent::SetTimeout(time) => self
                        .dyn_events
                        .push((query.uid, DynamicEvent::SetTimeout(*time))),
                };

                return QueryResponse::new(true);
            }
            DomInternalMessageType::ImportCss(css, for_hot_reload) => {
                self.window
                    .element_renderer
                    .load_css(&css, for_hot_reload.clone());
                return QueryResponse::new(true);
            }
            _ => (),
        };
        let mut response = QueryResponse::new(false);
        let elements = self.window.element_renderer.element_query(&query.selector);
        if elements.is_some() {
            for element in elements.unwrap() {
                response = match query.message {
                    DomInternalMessageType::PropertyChange(ref key, ref value) => {
                        self.window.element_renderer.emit_internal_event(
                            element,
                            XmlChangeEvent::PropertyChange(key.clone(), value.clone()),
                            false,
                        );
                        QueryResponse::new(true)
                    }
                    DomInternalMessageType::StyleChange(ref key, ref value) => {
                        self.window.element_renderer.emit_internal_event(
                            element,
                            XmlChangeEvent::StyleChange(key.clone(), value.clone()),
                            false,
                        );
                        QueryResponse::new(true)
                    }
                    DomInternalMessageType::RegisterEventListener(ref event_name) => {
                        self.window.element_renderer.register_event(
                            event_name.clone(),
                            element,
                            query.uid,
                        );
                        QueryResponse::new(true)
                    }
                    _ => {
                        // Handle other message types if needed
                        QueryResponse::new(false)
                    }
                };
            }
        }
        return response;
    }
}
