use iced::widget::center;

use crate::{
    dom::query::QueryResponse,
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        elements::{element_base::ElementBase, label::Label, library::AnyElement},
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub struct Center {
    children: Option<i32>,
    text: Option<String>,
    virtual_label: i32,
}

impl ElementBase for Center {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer, self_uid: i32) -> Self {
        if xml_element.children.len() != 1 && xml_element.text.trim().is_empty() {
            panic!("Center element must have exactly one child (or text inside)");
        }
        if xml_element.children.len() == 1 && !xml_element.text.trim().is_empty() {
            panic!("Center element must have either children or text, not both");
        }

        let virtual_label = renderer.init_element_virt(
            AnyElement::Label(Label::virt(xml_element.text.clone())),
            Some(xml_element.theme.clone()),
            self_uid,
        );

        if xml_element.text.trim().is_empty() {
            Self {
                children: Some(renderer.init_element_from_xml(&xml_element.children[0])),
                text: None,
                virtual_label,
            }
        } else {
            Self {
                children: None,
                text: Some(xml_element.text.clone()),
                virtual_label,
            }
        }
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        _: &'a ElementExtraData,
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

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        let mut result = QueryResponse::new(true);
        let mut elements_to_forward = Vec::new();
        match event {
            XmlChangeEvent::PropertyChange(key, newval) => {
                if key == "text" {
                    self.text = Some(newval.clone());
                    elements_to_forward.push(self.virtual_label);
                    Some((result, elements_to_forward, Vec::new()))
                } else {
                    None
                }
            }
            XmlChangeEvent::GetProperty(key) => {
                if key == "text" {
                    if self.text.is_some() {
                        result.data_str = self.text.clone();
                        Some((result, elements_to_forward, Vec::new()))
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
