use crate::{
    dom::{
        types::EventResponse,
        types::{DomInternalMessageType, EventListenerTypes},
    },
    elements_manager::{
        elements_manager::ElementsManager,
        mappings::ElementMappings,
        utils::{ElementEventResponse, ElementExtraData, EventListener, RendererEvent},
    },
    xml_engine::Message::{self},
    xml_struct::{
        elements::{element_base::ElementBase, library::ElementError},
        xml_element::XmlElement,
    },
};
use log::warn;

pub struct RadioButton {
    text: String,
    selection_id: String,
    choice: RadioElement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioElement {
    Element(i32), // Element(stringdb uid)
}

impl ElementBase for RadioButton {
    fn new(
        xml_element: Box<XmlElement>,
        renderer: &mut ElementMappings,
        _: i32,
    ) -> Result<Self, ElementError> {
        let mut text = xml_element.text.clone();

        if xml_element.text.is_empty() {
            warn!("Warning: created a <Radio /> element without text");
            text = String::from("No text");
        }

        let selection_id = xml_element.attributes.get("radio-id").ok_or(
            ElementError::AttributeIdRequiredForRadio(xml_element.clone()),
        )?;

        let id = xml_element.attributes.get("value").ok_or(
            ElementError::AttributeValueRequiredForRadio(xml_element.clone()),
        )?;

        let choice = RadioElement::Element(renderer.register_stringdb(id.clone()));

        if xml_element.attributes.get("selected").is_some() {
            renderer.set_radio_selection(id.clone(), choice);
        }

        return Ok(Self {
            selection_id: selection_id.clone(),
            text: text,
            choice: choice,
        });
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementsManager,
        datas: ElementExtraData,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();

        let mut ev_response = EventResponse::new(self_uid, EventListenerTypes::Select);

        let mut id = None;

        for event in events {
            match event.event_type {
                EventListenerTypes::Select => {
                    id = Some(event.event_uid);
                }
                _ => {}
            }
        }

        let current_choice = renderer
            .get_mappings()
            .get_radio_selection(self.selection_id.clone());

        let mut radio = iced::widget::radio::Radio::new(
            self.text.clone(),
            self.choice,
            current_choice,
            |choice| {
                ev_response.data_str = renderer.get_mappings().get_stringdb(match choice {
                    RadioElement::Element(id) => id,
                });
                return Message::DomEvent(id, ev_response);
            },
        );

        radio = radio
            .font(theme.font)
            .spacing(theme.spacing)
            .text_line_height(theme.line_height)
            .text_shaping(theme.shaping)
            .text_wrapping(theme.text_wrapping)
            .width(theme.width)
            .style(move |_, _| iced::widget::radio::Style {
                background: theme.background,
                border_color: theme.border_color,
                border_width: theme.border_width,
                dot_color: theme.icon_color,
                text_color: Some(theme.foreground_color),
            });

        if let Some(size) = theme.size {
            radio = radio.size(size);
        }

        if let Some(font_size) = theme.font_size {
            radio = radio.text_size(font_size);
        }

        return radio.into();
    }

    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        match event {
            DomInternalMessageType::PropertyChange(id, value) => match id.as_str() {
                "text" => {
                    self.text = value.clone();

                    return Some(ElementEventResponse::success());
                }
                _ => None,
            },
            DomInternalMessageType::GetProperty(id) => match id.as_str() {
                "text" => {
                    return Some(ElementEventResponse::success());
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn event_callback(
        &mut self,
        event_type: &EventListenerTypes,
        _: &EventResponse,
    ) -> Option<ElementEventResponse> {
        return match event_type {
            EventListenerTypes::Select => {
                return Some(ElementEventResponse::success().with_renderer_events(vec![
                    RendererEvent::RadioSelectionChange(self.selection_id.clone(), self.choice),
                ]));
            }
            _ => None,
        };
    }
}
