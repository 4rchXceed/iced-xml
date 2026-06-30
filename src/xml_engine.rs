use std::io::Cursor;

use crate::dom::events::DomInternalEvent;
use crate::dom::query::{Query, QueryResponse};
use crate::utilsfn;
use crate::xml_struct::parser::{XmlChangeEvent, XmlParser};
use crate::xml_struct::window::XmlWindow;
use quick_xml::Reader;
use utilsfn::safe_read_file;

const MAIN_WINDOW: &str = "src/main.xml";

#[derive(Debug, Clone)]
pub enum Message {
    DomEvent(i32),
}

pub struct XmlEngine {
    pub window: XmlWindow,
}

impl XmlEngine {
    pub fn new() -> Self {
        let content: String = safe_read_file(MAIN_WINDOW);
        let reader = Reader::from_reader(Cursor::new(content.into_bytes()));
        let window_parser = XmlParser::new(&mut reader.clone());
        let window = XmlWindow::new(window_parser.root);

        Self { window }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::DomEvent(event_uid) => {
                // self.window.emit_event(event_uid, &mut self.app_state);
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        return self.window.render();
    }

    pub fn client_events(&mut self, query: DomInternalEvent) -> QueryResponse {
        let response = match query {
            DomInternalEvent::PropertyChange(element_selector, key, value) => {
                let elements = self.window.element_renderer.element_query(element_selector);
                if elements.is_some() {
                    for element in elements.unwrap() {
                        self.window.element_renderer.emit_internal_event(
                            element,
                            XmlChangeEvent::PropertyChange(key.clone(), value.clone()),
                        );
                    }
                    QueryResponse { success: true }
                } else {
                    QueryResponse { success: false }
                }
            }
            DomInternalEvent::StyleChange(element_selector, key, value) => {
                let elements = self.window.element_renderer.element_query(element_selector);
                if elements.is_some() {
                    for element in elements.unwrap() {
                        self.window.element_renderer.emit_internal_event(
                            element,
                            XmlChangeEvent::StyleChange(key.clone(), value.clone()),
                        );
                    }
                    QueryResponse { success: true }
                } else {
                    QueryResponse { success: false }
                }
            }
        };
        return response;
    }
}
