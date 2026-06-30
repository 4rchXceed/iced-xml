use std::io::Cursor;

use crate::dom::events::{DomInternalMessageType, DomMessage};
use crate::dom::query::{EventResponse, Query, QueryResponse};
use crate::logger::fatal;
use crate::utilsfn;
use crate::xml_struct::parser::{XmlChangeEvent, XmlParser};
use crate::xml_struct::window::XmlWindow;
use quick_xml::Reader;
use utilsfn::safe_read_file;

#[derive(Debug, Clone)]
pub enum Message {
    DomEvent(i32, EventResponse),
}

pub struct XmlEngine {
    pub window: XmlWindow,
}

impl XmlEngine {
    pub fn new(xml: Vec<u8>) -> Self {
        let content: String = String::from_utf8(xml).expect("Failed to parse XML content as UTF-8");
        let reader = Reader::from_reader(Cursor::new(content.into_bytes()));
        let window_parser = XmlParser::new(&mut reader.clone());
        let window = XmlWindow::new(window_parser.root);

        Self { window }
    }

    pub fn update(&mut self, message: Message) -> Vec<(i32, EventResponse)> {
        self.window.fired_events.clear();
        match message {
            Message::DomEvent(event_uid, event_data) => {
                self.window.emit_event(event_uid, event_data);
            }
        };
        return self.window.fired_events.clone();
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        return self.window.render();
    }

    pub fn client_events(&mut self, query: &DomMessage) -> QueryResponse {
        let mut response = QueryResponse { success: false };
        let elements = self.window.element_renderer.element_query(&query.selector);
        if elements.is_some() {
            for element in elements.unwrap() {
                response = match query.message {
                    DomInternalMessageType::PropertyChange(ref key, ref value) => {
                        self.window.element_renderer.emit_internal_event(
                            element,
                            XmlChangeEvent::PropertyChange(key.clone(), value.clone()),
                        );
                        QueryResponse { success: true }
                    }
                    DomInternalMessageType::StyleChange(ref key, ref value) => {
                        self.window.element_renderer.emit_internal_event(
                            element,
                            XmlChangeEvent::StyleChange(key.clone(), value.clone()),
                        );
                        QueryResponse { success: true }
                    }
                    DomInternalMessageType::RegisterEventListener(ref event_name) => {
                        self.window.element_renderer.register_event(
                            event_name.clone(),
                            element,
                            query.uid,
                        );
                        QueryResponse { success: true }
                    }
                };
            }
        }
        return response;
    }
}
