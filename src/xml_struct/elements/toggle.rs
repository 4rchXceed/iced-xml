// Copy-paste template
use crate::{
    dom::{
        events::DomInternalMessageType,
        query_builder::{EventResponse, QueryResponse},
    },
    xml_engine::Message,
    xml_struct::{
        element_renderer::{
            ElementEventResponse, ElementExtraData, ElementRenderer, EventListener,
        },
        elements::element_base::ElementBase,
        parser::XmlElement,
    },
};

pub struct Toggle {
    toggled: bool,
    label: String,
}

impl ElementBase for Toggle {
    fn new(xml_element: &XmlElement, _: &mut ElementRenderer, _: i32) -> Self {
        Self {
            toggled: xml_element
                .attributes
                .get("toggled")
                .unwrap_or(&String::from("false"))
                == "true",
            label: xml_element
                .attributes
                .get("label")
                .unwrap_or(&String::from(""))
                .clone(),
        }
    }

    fn render<'a>(
        &'a self,
        _: &'a ElementRenderer,
        datas: ElementExtraData,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut handle_theme = theme.clone();

        if datas.flag_themes.contains_key("handle") {
            handle_theme = datas.flag_themes.get("handle").unwrap().clone();
        }

        let mut toggle = iced::widget::toggler::Toggler::new(self.toggled);

        toggle = toggle
            .font(theme.font)
            .label(self.label.as_str())
            .spacing(theme.spacing)
            .text_alignment(theme.toggle_text_alignment)
            .text_line_height(theme.line_height)
            .text_shaping(theme.shaping)
            .text_wrapping(theme.text_wrapping)
            .width(theme.width)
            .style(move |_, _| iced::widget::toggler::Style {
                background: theme.background,
                background_border_width: theme.border_width,
                background_border_color: theme.border_color,
                foreground: handle_theme.background,
                foreground_border_width: handle_theme.border_width,
                foreground_border_color: handle_theme.border_color,
                text_color: Some(theme.foreground_color),
                border_radius: Some(theme.border_radius),
                padding_ratio: theme.toggle_padding_ratio,
            });

        if theme.size.is_some() {
            toggle = toggle.size(theme.size.unwrap());
        }

        if theme.font_size.is_some() {
            toggle = toggle.text_size(theme.font_size.unwrap());
        }

        let mut event_id = None;
        for event in events {
            if event.event_type == "toggle" {
                event_id = Some(event.event_uid);
            }
        }

        toggle = toggle.on_toggle(move |toggled| {
            let mut event_response = EventResponse::new(self_uid, String::from("toggle"));
            event_response.data_bool = Some(toggled);
            return Message::DomEvent(event_id, event_response);
        });

        return toggle.into();
    }

    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        let mut query_response = QueryResponse::success();
        match event {
            DomInternalMessageType::PropertyChange(key, value) => match key.as_str() {
                "toggled" => {
                    self.toggled = value == "true";
                    return Some(ElementEventResponse::success());
                }
                "label" => {
                    self.label = value.clone();
                    return Some(ElementEventResponse::success());
                }
                _ => None,
            },
            DomInternalMessageType::GetProperty(key) => match key.as_str() {
                "toggled" => {
                    query_response.data_bool = Some(self.toggled);
                    return Some(ElementEventResponse::success());
                }
                "label" => {
                    query_response.data_str = Some(self.label.clone());
                    return Some(ElementEventResponse::success());
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn event_callback(
        &mut self,
        event_type: &String,
        event_response: &EventResponse,
    ) -> Option<ElementEventResponse> {
        match event_type.as_str() {
            "toggle" => {
                if event_response.data_bool.is_some() {
                    self.toggled = event_response.data_bool.unwrap();
                    return Some(ElementEventResponse::success());
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
