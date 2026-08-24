use std::panic;

use iced::{Border, Shadow};

use crate::{
    dom::{
        events::DomInternalMessageType,
        query::{EventResponse, QueryResponse},
    },
    xml_engine::Message,
    xml_struct::{
        element_renderer::{
            ElementEventResponse, ElementExtraData, ElementRenderer, EventListener,
        },
        elements::{element_base::ElementBase, label::Label, library::AnyElement},
        parser::XmlElement,
    },
};

pub struct Button {
    children: Vec<i32>,
    text: Option<String>,
    virtual_text: i32,
}

impl ElementBase for Button {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer, self_uid: i32) -> Self {
        let virtual_text = renderer.init_element_virt(
            AnyElement::Label(Label::virt(xml_element.text.clone())),
            Some(xml_element.theme.clone()),
            self_uid,
        );

        if !xml_element.text.trim().is_empty() {
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
                children.push(renderer.init_element_from_xml(child, self_uid));
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
        datas: ElementExtraData,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message> {
        let mut button_child: iced::widget::Column<'a, Message> = iced::widget::Column::new();
        for child in &self.children {
            button_child =
                button_child.push(renderer.render_element(*child, datas.child_data.clone()));
        }

        let mut button: iced::widget::Button<'a, Message> = iced::widget::Button::new(button_child);

        if self.text.is_some() {
            button = iced::widget::Button::new(
                renderer.render_element(self.virtual_text, datas.child_data.clone()),
            );
        }

        let theme = datas.default_theme.clone();

        let mut button = button
            .style(move |_, _| iced::widget::button::Style {
                background: Some(theme.background),
                text_color: theme.foreground_color,
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
                        Some(event.event_uid),
                        EventResponse::new(self_uid, event.event_type.clone()),
                    ));
                }
                _ => (),
            }
        }

        return button.into();
    }

    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        // returns (query_response, elementsToForwardTheEvent)
        match event {
            DomInternalMessageType::PropertyChange(property, new_val) => {
                return match property.as_str() {
                    "text" => {
                        self.text = Some(new_val.clone());
                        Some(
                            ElementEventResponse::success()
                                .with_forward_to(vec![self.virtual_text]),
                        )
                    }
                    _ => None,
                };
            }
            DomInternalMessageType::GetProperty(property) => {
                return match property.as_str() {
                    "text" => {
                        if self.text.is_some() {
                            Some(ElementEventResponse::new(
                                QueryResponse::success()
                                    .with_data_str(self.text.as_ref().unwrap().clone()),
                            ))
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
