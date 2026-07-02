// Copy-paste template
use crate::{
    dom::query::{EventResponse, QueryResponse},
    xml_engine::Message,
    xml_struct::{
        elements::{ElementRenderer, EventListener, element_base::ElementBase},
        parser::{XmlChangeEvent, XmlElement},
        theming::XmlTheme,
    },
};

pub struct Checkbox {
    // If you have children, store them here
    // children: Vec<i32>,
    checked: bool,
    text: Option<String>,
}

impl ElementBase for Checkbox {
    fn new(xml_element: &XmlElement, _: &mut ElementRenderer, _: i32) -> Self {
        // If it supports children, initialize them here with renderer.init_element
        // let mut children: Vec<i32> = Vec::new();
        // for child in &xml_element.children {
        //     children.push(renderer.init_element(child));
        // }
        let checked = xml_element.attributes.get("checked").is_some();
        let text: String = xml_element.text.clone();
        let mut maybe_text: Option<String> = None;
        if !text.is_empty() {
            maybe_text = Some(text);
        }
        Self {
            checked: checked,
            text: maybe_text,
        }
    }

    fn render<'a>(
        &self,
        _: &'a ElementRenderer,
        theme: &'a XmlTheme,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message> {
        // Declare your element
        // let mut container: iced::widget::Column<'a, Message> = iced::widget::Column::new();
        let mut checkbox: iced::widget::Checkbox<'a, Message> =
            iced::widget::checkbox::Checkbox::new(self.checked);

        checkbox = checkbox
            .font(theme.font)
            .spacing(theme.spacing)
            .text_line_height(theme.line_height)
            .text_shaping(theme.shaping)
            .text_wrapping(theme.text_wrapping)
            .width(theme.width);

        if let Some(size) = theme.size {
            checkbox = checkbox.size(size);
        }
        if let Some(font_size) = theme.font_size {
            checkbox = checkbox.text_size(font_size);
        }

        if let Some(text) = &self.text {
            checkbox = checkbox.label(text.clone());
        }

        if let Some(icon) = &theme.checkbox_icon {
            checkbox = checkbox.icon(icon.clone());
        }

        // Register any events here
        let me = self_uid.clone();
        for event in events {
            match event.event_type.as_str() {
                "checked" => {
                    checkbox = checkbox.on_toggle(move |_| {
                        return Message::DomEvent(event.event_uid, EventResponse::new(me));
                    });
                }
                _ => (),
            }
        }

        return checkbox.into();
    }

    fn process_event(&mut self, event: &XmlChangeEvent) -> Option<(QueryResponse, Vec<i32>)> {
        match event {
            XmlChangeEvent::PropertyChange(key, newval) => {
                if key == "text" {
                    self.text = Some(newval.clone());
                    return Some((QueryResponse::new(true), Vec::new()));
                } else if key == "checked" {
                    self.checked = newval == "true";
                    return Some((QueryResponse::new(true), Vec::new()));
                } else {
                    return None;
                }
            }
            XmlChangeEvent::GetProperty(key) => {
                if key == "checked" {
                    let mut qr = QueryResponse::new(true);
                    qr.data_bool = Some(self.checked);
                    return Some((qr, Vec::new()));
                } else {
                    return None;
                }
            }
            XmlChangeEvent::EventFired(event_type) => {
                if event_type == "checked" {
                    self.checked = !self.checked;
                    return Some((QueryResponse::new(true), Vec::new()));
                } else {
                    return None;
                }
            }
            _ => None,
        }
    }
}
