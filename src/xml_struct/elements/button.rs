use iced::{Background, Border, Shadow, widget::text};

use crate::{
    dom::query::EventResponse,
    logger::fatal,
    xml_engine::Message,
    xml_struct::{
        elements::{ElementRenderer, EventListener, element_base::ElementBase},
        parser::{XmlChangeEvent, XmlElement, XmlTheme},
    },
};

pub struct Button {
    children: Vec<i32>,
    text: Option<String>,
}

impl ElementBase for Button {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer) -> Self {
        if !xml_element.text.is_empty() {
            if xml_element.children.is_empty() {
                Self {
                    children: Vec::new(),
                    text: Some(xml_element.text.clone()),
                }
            } else {
                fatal(
                    format!(
                        "Button with text cannot have children: {}",
                        xml_element.text
                    )
                    .as_str(),
                );
                Self {
                    children: Vec::new(),
                    text: None,
                }
            }
        } else {
            let mut children: Vec<i32> = Vec::new();
            for child in &xml_element.children {
                children.push(renderer.init_element(child));
            }
            Self {
                children: children,
                text: None,
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
            button = iced::widget::Button::new(text(self.text.clone().unwrap()));
        }

        let theme = theme.clone();

        let mut button = button.style(move |_, _| iced::widget::button::Style {
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
        });

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

    fn process_event(&mut self, event: &XmlChangeEvent) {
        match event {
            XmlChangeEvent::PropertyChange(property, new_val) => {
                match property.as_str() {
                    "text" => self.text = Some(new_val.clone()),
                    _ => (),
                };
            }
            _ => (),
        }
    }
}
