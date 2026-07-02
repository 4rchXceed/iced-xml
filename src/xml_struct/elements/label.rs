use iced::widget::text;

use crate::{
    dom::query::QueryResponse,
    logger::fatal,
    xml_engine::Message,
    xml_struct::{
        elements::{ElementRenderer, EventListener, element_base::ElementBase},
        parser::{XmlChangeEvent, XmlElement, XmlTheme},
    },
};

pub struct Label {
    text: String,
}

impl ElementBase for Label {
    fn new(xml_element: &XmlElement, _: &mut ElementRenderer) -> Self {
        if xml_element.children.len() > 0 {
            fatal("<Label> elements cannot have children");
        }
        Self {
            text: xml_element.text.clone(),
        }
    }

    fn render<'a>(
        &self,
        _: &'a ElementRenderer,
        theme: &'a XmlTheme,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        let text_element = text(self.text.clone());

        let text_element = text_element.style(move |_| iced::widget::text::Style {
            color: Some(theme.text_color),
        });

        let text_element = text_element.height(theme.height).width(theme.width);

        return text_element.into();
    }

    fn process_event(&mut self, event: &XmlChangeEvent) -> Option<QueryResponse> {
        let mut query_response = QueryResponse::new(true);
        match event {
            XmlChangeEvent::PropertyChange(property, new_val) => {
                return match property.as_str() {
                    "text" => {
                        self.text = new_val.clone();
                        Some(query_response)
                    }
                    _ => None,
                };
            }
            XmlChangeEvent::GetProperty(property) => {
                return match property.as_str() {
                    "text" => {
                        query_response.data_str = Some(self.text.clone());
                        Some(query_response)
                    }
                    _ => None,
                };
            }
            _ => None,
        }
    }
}
