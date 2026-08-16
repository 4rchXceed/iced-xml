// Copy-paste template
use crate::{
    dom::query::{EventResponse, QueryResponse},
    xml_engine::Message,
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        elements::element_base::ElementBase,
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub struct Input {
    placeholder: String,
    value: String,
    is_secured: bool,
}

impl ElementBase for Input {
    fn new(xml_element: &XmlElement, _: &mut ElementRenderer, _: i32) -> Self {
        let placeholder = xml_element
            .attributes
            .get("placeholder")
            .unwrap_or(&String::from("Enter something here..."))
            .to_string();
        let value = xml_element.text.clone();
        let is_secured = xml_element.attributes.contains_key("secure");

        Self {
            placeholder: placeholder,
            value: value,
            is_secured,
        }
    }

    fn render<'a>(
        &self,
        _: &'a ElementRenderer,
        datas: ElementExtraData,
        events: Vec<&'a EventListener>,
        self_id: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();

        let mut input = iced::widget::text_input(&self.placeholder, &self.value);

        if self.is_secured {
            input = input.secure(true);
        }

        let me = self_id;

        input = input
            .align_x(theme.align_x)
            .font(theme.font)
            .line_height(theme.line_height)
            .padding(theme.padding)
            .width(theme.width)
            .style(move |_, _| iced::widget::text_input::Style {
                background: iced::Background::Color(theme.background_color),
                border: iced::Border {
                    color: theme.border_color,
                    width: theme.border_width,
                    radius: theme.border_radius,
                },
                icon: theme.icon_color,
                placeholder: theme.input_placeholder_color,
                value: theme.foreground_color,
                selection: theme.selection_color,
            })
            .on_input(move |input| {
                let mut event_response = EventResponse::new(me, String::from("input"));
                event_response.data_str = Some(input);
                return Message::DomEvent(-1, event_response);
            })
            .on_paste(move |input| {
                let mut event_response = EventResponse::new(me, String::from("paste"));
                event_response.data_str = Some(input);
                return Message::DomEvent(-1, event_response);
            })
            .on_submit(Message::DomEvent(
                -1,
                EventResponse::new(me, String::from("submit")),
            ));

        if theme.select_icon.is_some() {
            input = input.icon(theme.select_icon.unwrap());
        }

        if theme.size.is_some() {
            input = input.size(theme.size.unwrap());
        }

        for event in events {
            match event.event_type.as_str() {
                "input" => {
                    input = input.on_input(move |input| {
                        let mut event_response = EventResponse::new(me, String::from("input"));
                        event_response.data_str = Some(input);
                        return Message::DomEvent(event.event_uid, event_response);
                    });
                }
                "paste" => {
                    input = input.on_paste(move |input| {
                        let mut event_response = EventResponse::new(me, String::from("paste"));
                        event_response.data_str = Some(input);
                        return Message::DomEvent(event.event_uid, event_response);
                    });
                }
                "submit" => {
                    input = input.on_submit(Message::DomEvent(
                        event.event_uid,
                        EventResponse::new(event.event_uid, String::from("submit")),
                    ));
                }
                _ => (),
            }
        }

        return input.into();
    }

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        let mut query_response = QueryResponse::new(true);
        let default_response = Some((query_response.clone(), Vec::new(), Vec::new()));
        match event {
            XmlChangeEvent::EventFired(event_type, response) => match event_type.as_str() {
                "input" => {
                    if let Some(data_str) = &response.data_str {
                        self.value = data_str.clone();
                    }
                    default_response
                }
                "paste" => {
                    if let Some(data_str) = &response.data_str {
                        self.value = data_str.clone();
                    }
                    default_response
                }
                "submit" => default_response,
                _ => None,
            },
            XmlChangeEvent::PropertyChange(key, value) => match key.as_str() {
                "placeholder" => {
                    self.placeholder = value.clone();
                    return default_response;
                }
                "value" => {
                    self.value = value.clone();
                    return default_response;
                }
                "secure" => {
                    self.is_secured = value == "true";
                    return default_response;
                }
                _ => None,
            },
            XmlChangeEvent::GetProperty(key) => match key.as_str() {
                "placeholder" => {
                    query_response.data_str = Some(self.placeholder.clone());
                    Some((query_response, Vec::new(), Vec::new()))
                }
                "value" => {
                    query_response.data_str = Some(self.value.clone());
                    Some((query_response, Vec::new(), Vec::new()))
                }
                "secure" => {
                    query_response.data_str = Some(self.is_secured.to_string());
                    Some((query_response, Vec::new(), Vec::new()))
                }
                _ => None,
            },
            _ => None,
        }
    }
}
