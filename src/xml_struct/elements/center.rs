use iced::widget::center;

use crate::{
    dom::query::QueryResponse,
    xml_engine::Message,
    xml_struct::{
        elements::{ElementRenderer, EventListener, element_base::ElementBase},
        parser::{XmlChangeEvent, XmlElement, XmlTheme},
    },
};

pub struct Center {
    children: Option<i32>,
    text: Option<String>,
}

impl ElementBase for Center {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer) -> Self {
        if xml_element.children.len() != 1 && xml_element.text.is_empty() {
            panic!("Center element must have exactly one child (or text inside)");
        }
        if xml_element.children.len() == 1 && !xml_element.text.is_empty() {
            panic!("Center element must have either children or text, not both");
        }

        if xml_element.text.is_empty() {
            Self {
                children: Some(renderer.init_element(&xml_element.children[0])),
                text: None,
            }
        } else {
            Self {
                children: None,
                text: Some(xml_element.text.clone()),
            }
        }
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        _: &'a XmlTheme,
        _: Vec<&'a EventListener>,
        _: i32,
    ) -> iced::Element<'a, Message> {
        if let Some(children) = &self.children {
            let center: iced::widget::Container<'a, Message> =
                center(renderer.render_element(*children));
            return center.into();
        } else {
            let center = center(iced::widget::text(self.text.clone().unwrap()));
            return center.into();
        }
    }

    fn process_event(&mut self, event: &XmlChangeEvent) -> Option<QueryResponse> {
        let mut result = QueryResponse::new(true);
        match event {
            XmlChangeEvent::PropertyChange(key, newval) => {
                if key == "text" {
                    self.text = Some(newval.clone());
                }
                Some(result)
            }
            XmlChangeEvent::GetProperty(key) => {
                if key == "text" {
                    if self.text.is_some() {
                        result.data_str = self.text.clone();
                        Some(result)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
