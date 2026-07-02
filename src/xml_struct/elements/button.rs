use std::panic;

use iced::{Background, Border, Shadow};

use crate::{
    dom::query::{EventResponse, QueryResponse},
    xml_engine::Message,
    xml_struct::{
        elements::{
            ElementRenderer, EventListener, element_base::ElementBase, label::Label,
            library::AnyElement,
        },
        parser::{XmlChangeEvent, XmlElement},
        theming::XmlTheme,
    },
};

pub struct Button {
    children: Vec<i32>,
    text: Option<String>,
    virtual_text: i32,
}

impl ElementBase for Button {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer) -> Self {
        let virtual_text = renderer.init_element(
            AnyElement::Label(Label::virt(xml_element.text.clone())),
            None,
            Some(xml_element.theme.clone()),
        );

        if !xml_element.text.is_empty() {
            if xml_element.children.is_empty() {
                Self {
                    children: Vec::new(),
                    text: Some(xml_element.text.clone()),
                    virtual_text: virtual_text,
                }
            } else {
                panic!(
                    "Button with text cannot have children: {}",
                    xml_element.text
                );
            }
        } else {
            let mut children: Vec<i32> = Vec::new();
            for child in &xml_element.children {
                children.push(renderer.init_element_from_xml(child));
            }

            Self {
                children: children,
                text: None,
                virtual_text: virtual_text,
            }
        }
    }
    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        theme: &'a XmlTheme,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message> {
        let mut button_child: iced::widget::Column<'a, Message> = iced::widget::Column::new();
        for child in &self.children {
            button_child = button_child.push(renderer.render_element(*child));
        }

        let mut button: iced::widget::Button<'a, Message> = iced::widget::Button::new(button_child);

        if self.text.is_some() {
            button = iced::widget::Button::new(renderer.render_element(self.virtual_text));
        }

        let theme = theme.clone();

        let mut button = button
            .style(move |_, _| iced::widget::button::Style {
                background: Some(Background::Color(theme.background_color)),
                text_color: theme.text_color,
                border: Border {
                    color: theme.border_color,
                    radius: theme.border_radius,
                    width: theme.border_width,
                    ..Border::default()
                },
                shadow: Shadow {
                    color: theme.shadow_color,
                    blur_radius: theme.shadow_blur_radius,
                    offset: theme.shadow_offset,
                    ..Shadow::default()
                },
                snap: theme.snap,
                ..Default::default()
            })
            .clip(theme.clip)
            .height(theme.height)
            .padding(theme.padding)
            .width(theme.width);

        for event in events {
            match event.event_type.as_str() {
                "click" => {
                    button = button.on_press(Message::DomEvent(
                        event.event_uid,
                        EventResponse::new(self_uid),
                    ));
                }
                _ => (),
            }
        }

        return button.into();
    }

    fn process_event(&mut self, event: &XmlChangeEvent) -> Option<(QueryResponse, Vec<i32>)> {
        // returns (query_response, elementsToForwardTheEvent)
        let mut query_response = QueryResponse::new(true);
        let mut elements_to_forward = Vec::new();
        match event {
            XmlChangeEvent::PropertyChange(property, new_val) => {
                return match property.as_str() {
                    "text" => {
                        self.text = Some(new_val.clone());
                        elements_to_forward.push(self.virtual_text);
                        Some((query_response, elements_to_forward))
                    }
                    _ => None,
                };
            }
            XmlChangeEvent::GetProperty(property) => {
                return match property.as_str() {
                    "text" => {
                        if self.text.is_some() {
                            query_response.data_str = self.text.clone();
                            Some((query_response, elements_to_forward))
                        } else {
                            None
                        }
                    }
                    _ => None,
                };
            }
            _ => None,
        }
    }
}
