// Copy-paste template
use crate::{
    dom::{
        events::{DomInternalMessageType, EventListenerTypes},
        query_builder::{EventResponse, QueryResponse},
    },
    xml_engine::Message,
    xml_struct::{
        element_renderer::{
            ElementEventResponse, ElementExtraData, ElementRenderer, EventListener,
        },
        elements::{element_base::ElementBase, library::ElementError},
        parser::XmlElement,
    },
};

pub struct Checkbox {
    // If you have children, store them here
    // children: Vec<i32>,
    checked: bool,
    text: Option<String>,
}

impl ElementBase for Checkbox {
    fn new(
        xml_element: &XmlElement,
        _: &mut ElementRenderer,
        _: i32,
    ) -> Result<Self, ElementError> {
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
        return Ok(Self {
            checked: checked,
            text: maybe_text,
        });
    }

    fn render<'a>(
        &self,
        _: &'a ElementRenderer,
        datas: ElementExtraData,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message> {
        // Declare your element
        // let mut container: iced::widget::Column<'a, Message> = iced::widget::Column::new();
        let mut checkbox: iced::widget::Checkbox<'a, Message> =
            iced::widget::checkbox::Checkbox::new(self.checked);
        let mut theme = datas.default_theme.clone();
        if self.checked {
            if datas.flag_themes.contains_key("checked") {
                theme = datas.flag_themes.get("checked").unwrap().clone();
            }
        } else {
            if datas.flag_themes.contains_key("unchecked") {
                theme = datas.flag_themes.get("unchecked").unwrap().clone();
            }
        }

        checkbox = checkbox
            .font(theme.font)
            .spacing(theme.spacing)
            .text_line_height(theme.line_height)
            .text_shaping(theme.shaping)
            .text_wrapping(theme.text_wrapping)
            .width(theme.width)
            .style(move |_, _| iced::widget::checkbox::Style {
                background: theme.background,
                border: iced::Border {
                    color: theme.border_color,
                    width: theme.border_width,
                    radius: theme.border_radius,
                },
                icon_color: theme.icon_color,
                text_color: Some(theme.foreground_color),
            });

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
        checkbox = checkbox.on_toggle(move |_| {
            return Message::DomEvent(None, EventResponse::new(me, EventListenerTypes::Checked));
        });
        for event in events {
            match event.event_type {
                EventListenerTypes::Checked => {
                    checkbox = checkbox.on_toggle(move |_| {
                        return Message::DomEvent(
                            Some(event.event_uid),
                            EventResponse::new(me, event.event_type.clone()),
                        );
                    });
                }
                _ => (),
            }
        }

        return checkbox.into();
    }

    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        match event {
            DomInternalMessageType::PropertyChange(key, newval) => {
                if key == "text" {
                    self.text = Some(newval.clone());
                    return Some(ElementEventResponse::success());
                } else if key == "checked" {
                    self.checked = newval == "true";
                    return Some(ElementEventResponse::success());
                } else {
                    return None;
                }
            }
            DomInternalMessageType::GetProperty(key) => {
                if key == "checked" {
                    return Some(ElementEventResponse::new(
                        QueryResponse::success().with_data_bool(self.checked),
                    ));
                } else {
                    return None;
                }
            }
            _ => None,
        }
    }

    fn event_callback(
        &mut self,
        event_type: &EventListenerTypes,
        _: &EventResponse,
    ) -> Option<ElementEventResponse> {
        match event_type {
            EventListenerTypes::Checked => {
                self.checked = !self.checked;
                return Some(ElementEventResponse::success());
            }
            _ => None,
        }
    }
}
