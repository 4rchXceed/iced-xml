use crate::{
    dom::query::{EventResponse, QueryResponse},
    xml_engine::Message::{self},
    xml_struct::{
        element_renderer::{ElementExtraData, ElementRenderer, EventListener, RendererEvent},
        elements::element_base::ElementBase,
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub struct RadioButton {
    text: String,
    selection_id: String,
    choice: RadioElement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioElement {
    Element(i32), // Element(uid)
}

impl ElementBase for RadioButton {
    fn new(xml_element: &XmlElement, renderer: &mut ElementRenderer, _: i32) -> Self {
        if xml_element.attributes.get("radio-id").is_none() {
            panic!("Attribute id is required on <Radio />");
        }
        if xml_element.attributes.get("selection-id").is_none() {
            panic!("Attribute selection-id is required on <Radio />");
        }
        let mut text = xml_element.text.clone();
        if xml_element.text.is_empty() {
            println!("Warning: created a <Radio /> element without text");
            text = String::from("No text");
        }
        let id = xml_element.attributes.get("radio-id").unwrap();
        let choice = RadioElement::Element(renderer.register_stringdb(id.clone()));

        if xml_element.attributes.get("selected").is_some() {
            renderer.set_radio_selection(id.clone(), choice);
        }

        Self {
            selection_id: xml_element.attributes.get("selection-id").unwrap().clone(),
            text: text,
            choice: choice,
        }
    }

    fn render<'a>(
        &self,
        renderer: &'a ElementRenderer,
        datas: ElementExtraData,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();
        let mut ev_response = EventResponse::new(self_uid, String::from("select"));
        let mut id = -1;
        for event in events {
            match event.event_type.as_str() {
                "select" => {
                    id = event.event_uid;
                }
                _ => {}
            }
        }

        let current_choice = renderer.get_radio_selection(self.selection_id.clone());

        let mut radio = iced::widget::radio::Radio::new(
            self.text.clone(),
            self.choice,
            current_choice,
            |choice| {
                ev_response.data_str = renderer.get_stringdb(match choice {
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
        if theme.size.is_some() {
            radio = radio.size(theme.size.unwrap());
        }
        if theme.font_size.is_some() {
            radio = radio.text_size(theme.font_size.unwrap());
        }

        return radio.into();
    }

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        match event {
            XmlChangeEvent::PropertyChange(id, value) => match id.as_str() {
                "text" => {
                    self.text = value.clone();
                    return Some((QueryResponse::new(true), Vec::new(), Vec::new()));
                }
                _ => None,
            },
            XmlChangeEvent::GetProperty(id) => match id.as_str() {
                "text" => {
                    let query_response = QueryResponse::new(true);
                    return Some((query_response, Vec::new(), Vec::new()));
                }
                _ => None,
            },
            XmlChangeEvent::EventFired(event_type, _) => match event_type.as_str() {
                "select" => {
                    let query_response = QueryResponse::new(true);
                    return Some((
                        query_response,
                        Vec::new(),
                        Vec::from([RendererEvent::RadioSelectionChange(
                            self.selection_id.clone(),
                            self.choice,
                        )]),
                    ));
                }
                _ => None,
            },
            _ => None,
        }
    }
}
