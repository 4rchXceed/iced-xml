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

pub struct Input {
    placeholder: String,
    value: String,
    is_secured: bool,
}

impl ElementBase for Input {
    fn new(
        xml_element: &XmlElement,
        _: &mut ElementRenderer,
        _: i32,
    ) -> Result<Self, ElementError> {
        let placeholder = xml_element
            .attributes
            .get("placeholder")
            .unwrap_or(&String::from("Enter something here..."))
            .to_string();
        let value = xml_element.text.clone();
        let is_secured = xml_element.attributes.contains_key("secure");

        return Ok(Self {
            placeholder: placeholder,
            value: value,
            is_secured,
        });
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
                background: theme.background,
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
                let mut event_response = EventResponse::new(me, EventListenerTypes::Input);
                event_response.data_str = Some(input);
                return Message::DomEvent(None, event_response);
            })
            .on_paste(move |input| {
                let mut event_response = EventResponse::new(me, EventListenerTypes::Paste);
                event_response.data_str = Some(input);
                return Message::DomEvent(None, event_response);
            })
            .on_submit(Message::DomEvent(
                None,
                EventResponse::new(me, EventListenerTypes::Submit),
            ));

        if theme.select_icon.is_some() {
            input = input.icon(theme.select_icon.unwrap());
        }

        if theme.size.is_some() {
            input = input.size(theme.size.unwrap());
        }

        for event in events {
            match event.event_type {
                EventListenerTypes::Input => {
                    input = input.on_input(move |input| {
                        let mut event_response = EventResponse::new(me, event.event_type.clone());
                        event_response.data_str = Some(input);
                        return Message::DomEvent(Some(event.event_uid), event_response);
                    });
                }
                EventListenerTypes::Paste => {
                    input = input.on_paste(move |input| {
                        let mut event_response = EventResponse::new(me, event.event_type.clone());
                        event_response.data_str = Some(input);
                        return Message::DomEvent(Some(event.event_uid), event_response);
                    });
                }
                EventListenerTypes::Submit => {
                    input = input.on_submit(Message::DomEvent(
                        Some(event.event_uid),
                        EventResponse::new(event.event_uid, event.event_type.clone()),
                    ));
                }
                _ => (),
            }
        }

        return input.into();
    }

    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        match event {
            DomInternalMessageType::PropertyChange(key, value) => match key.as_str() {
                "placeholder" => {
                    self.placeholder = value.clone();
                    return Some(ElementEventResponse::success());
                }
                "value" => {
                    self.value = value.clone();
                    return Some(ElementEventResponse::success());
                }
                "secure" => {
                    self.is_secured = value == "true";
                    return Some(ElementEventResponse::success());
                }
                _ => None,
            },
            DomInternalMessageType::GetProperty(key) => match key.as_str() {
                "placeholder" => Some(ElementEventResponse::new(
                    QueryResponse::success().with_data_str(self.placeholder.clone()),
                )),
                "value" => Some(ElementEventResponse::new(
                    QueryResponse::success().with_data_str(self.value.clone()),
                )),
                "secure" => Some(ElementEventResponse::new(
                    QueryResponse::success().with_data_bool(self.is_secured),
                )),
                _ => None,
            },
            _ => None,
        }
    }

    fn event_callback(
        &mut self,
        event_type: &EventListenerTypes,
        event_response: &EventResponse,
    ) -> Option<ElementEventResponse> {
        match event_type {
            EventListenerTypes::Input => {
                if event_response.data_str.is_some() {
                    self.value = event_response.data_str.as_ref().unwrap().clone();
                    Some(ElementEventResponse::success())
                } else {
                    None
                }
            }
            EventListenerTypes::Paste => {
                if event_response.data_str.is_some() {
                    self.value = event_response.data_str.as_ref().unwrap().clone();
                    Some(ElementEventResponse::success())
                } else {
                    None
                }
            }
            EventListenerTypes::Submit => Some(ElementEventResponse::success()),
            _ => None,
        }
    }
}
