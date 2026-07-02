use iced::widget::text;

use crate::{
    dom::query::QueryResponse,
    xml_engine::Message,
    xml_struct::{
        elements::{ElementRenderer, EventListener, element_base::ElementBase},
        parser::{XmlChangeEvent, XmlElement},
        theming::XmlTheme,
    },
};

pub struct Label {
    pub text: String,
}

impl Label {
    pub fn virt(text: String) -> Self {
        Self { text }
    }
}

impl ElementBase for Label {
    fn new(xml_element: &XmlElement, _: &mut ElementRenderer, _: i32) -> Self {
        if xml_element.children.len() > 0 {
            panic!("<Label> elements cannot have children");
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

        let mut text_element = text_element
            .height(theme.height)
            .width(theme.width)
            .font(theme.font)
            .shaping(theme.shaping)
            .wrapping(theme.text_wrapping)
            .line_height(theme.line_height);

        if let Some(font_size) = theme.font_size {
            text_element = text_element.size(font_size);
        }

        if theme.center {
            text_element = text_element.center();
        }

        return text_element.into();
    }

    fn process_event(&mut self, event: &XmlChangeEvent) -> Option<(QueryResponse, Vec<i32>)> {
        let mut query_response = QueryResponse::new(true);
        match event {
            XmlChangeEvent::PropertyChange(property, new_val) => {
                return match property.as_str() {
                    "text" => {
                        self.text = new_val.clone();
                        Some((query_response, Vec::new()))
                    }
                    _ => None,
                };
            }
            XmlChangeEvent::GetProperty(property) => {
                return match property.as_str() {
                    "text" => {
                        query_response.data_str = Some(self.text.clone());
                        Some((query_response, Vec::new()))
                    }
                    _ => None,
                };
            }
            _ => None,
        }
    }
}
