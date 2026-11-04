use std::fmt;

use iced::{Border, Shadow};
use log::warn;

use crate::{
    dom::{
        types::{CustomElementEvent, EventResponse, QueryResponse},
        types::{DomInternalMessageType, EventListenerTypes},
    },
    elements_manager::{
        elements_manager::ElementsManager,
        mappings::ElementMappings,
        utils::{ElementEventResponse, ElementExtraData, EventListener},
    },
    xml_engine::Message,
    xml_struct::{
        elements::{element_base::ElementBase, library::ElementError},
        xml_element::XmlElement,
    },
};

#[derive(Clone, Debug, Hash)]
struct SelectEntry {
    text: String,
    id: String,
}

impl fmt::Display for SelectEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.text.as_str());
    }
}

pub struct Select {
    // If you have children, store them here
    // children: Vec<i32>,
    state: iced::widget::combo_box::State<SelectEntry>, // id => Text
    placeholder: String,
    selected: Option<SelectEntry>,
}

impl ElementBase for Select {
    fn new(
        xml_element: Box<XmlElement>,
        _: &mut ElementMappings,
        _: i32,
    ) -> Result<Self, ElementError> {
        // If it supports children, initialize them here with renderer.init_element
        let mut options: Vec<SelectEntry> = Vec::new();
        let mut selected_id: Option<usize> = None;

        if !xml_element.text.trim().is_empty() {
            warn!(
                "<Select /> element has text content, but it will be ignored. Only <Option /> children are allowed"
            );
        }

        for child in &xml_element.children {
            if child.tag == "Option" {
                let option_id = if let Some(v) = child.attributes.get("value") {
                    v
                } else {
                    println!(
                        "Warning <Option /> element doesn't have a value=\"\" attribute, using text as id, but it's not recommended"
                    );
                    child.text.trim()
                };

                let entry = SelectEntry {
                    text: child.text.trim().to_string(),
                    id: option_id.to_string(),
                };

                if child
                    .attributes
                    .get("selected")
                    .unwrap_or(&"false".to_string())
                    == "true"
                {
                    selected_id = Some(options.len());
                }

                options.push(entry);
            } else {
                return Err(ElementError::OnlyOptionElementAllowedInComboBox(
                    xml_element.clone(),
                    child.clone(),
                ));
            }
        }

        let placeholder_op = xml_element.attributes.get("placeholder");

        let placeholder: String = if let Some(ph) = placeholder_op {
            ph.clone()
        } else {
            println!("Using auto-generated placeholder for <Select /> is not recommended");
            "Please choose an option".to_string()
        };

        let mut selected: Option<&SelectEntry> = None;

        if let Some(selected_id) = selected_id {
            selected = options.get(selected_id);
        }

        return Ok(Self {
            state: iced::widget::combo_box::State::with_selection(options.clone(), selected),
            placeholder: placeholder,
            selected: selected.cloned(),
        });
    }

    fn render<'a>(
        &'a self,
        _: &'a ElementsManager,
        datas: ElementExtraData,
        events: Vec<&'a EventListener>,
        self_uid: i32,
    ) -> iced::Element<'a, Message> {
        let theme = datas.default_theme.clone();

        let mut select_event_uid: Option<i32> = None;

        for event in &events {
            match event.event_type {
                EventListenerTypes::Select => {
                    select_event_uid = Some(event.event_uid);
                }
                _ => (),
            }
        }

        let mut combo_box: iced::widget::combo_box::ComboBox<'a, SelectEntry, Message> =
            iced::widget::combo_box::ComboBox::new(
                &self.state,
                &self.placeholder,
                self.selected.as_ref(),
                move |selected_entry| {
                    let mut event_response =
                        EventResponse::new(self_uid, EventListenerTypes::Select);

                    event_response.data_str = Some(selected_entry.id);

                    if let Some(event_uid) = select_event_uid {
                        return Message::DomEvent(Some(event_uid), event_response);
                    } else {
                        return Message::DomEvent(None, event_response);
                    }
                },
            );

        combo_box = combo_box
            .font(theme.font)
            .line_height(theme.line_height)
            .menu_height(theme.select_menu_height)
            .padding(theme.padding)
            .text_shaping(theme.shaping)
            .width(theme.width);

        if let Some(size) = theme.size {
            combo_box = combo_box.size(size);
        }

        if let Some(input_theme) = datas.flag_themes.get("input").cloned() {
            combo_box = combo_box.input_style(move |_, _| iced::widget::text_input::Style {
                background: input_theme.background,
                border: Border {
                    color: input_theme.border_color,
                    radius: input_theme.border_radius,
                    width: input_theme.border_width,
                },
                icon: input_theme.icon_color,
                placeholder: input_theme.input_placeholder_color,
                value: input_theme.foreground_color,
                selection: input_theme.selection_color,
            });
        }

        if let Some(menu_theme) = datas.flag_themes.get("select-menu").cloned() {
            combo_box = combo_box.menu_style(move |_| iced::overlay::menu::Style {
                background: menu_theme.background, // TODO: Add support for gradient backgrounds
                border: Border {
                    color: menu_theme.border_color,
                    radius: menu_theme.border_radius,
                    width: menu_theme.border_width,
                },
                text_color: menu_theme.foreground_color,
                selected_background: menu_theme.selected_background,
                selected_text_color: menu_theme.selected_text_color,
                shadow: Shadow {
                    color: menu_theme.shadow_color,
                    offset: menu_theme.shadow_offset,
                    blur_radius: menu_theme.shadow_blur_radius,
                },
            });
        }

        if let Some(icon) = theme.select_icon {
            combo_box = combo_box.icon(icon);
        }

        for event in events {
            match event.event_type {
                EventListenerTypes::OnClose => {
                    combo_box = combo_box.on_close(Message::DomEvent(
                        Some(event.event_uid),
                        EventResponse::new(self_uid, event.event_type.clone()),
                    ));
                }
                EventListenerTypes::OnOpen => {
                    combo_box = combo_box.on_open(Message::DomEvent(
                        Some(event.event_uid),
                        EventResponse::new(self_uid, event.event_type.clone()),
                    ));
                }
                EventListenerTypes::OnInput => {
                    let event_uid = Some(event.event_uid);
                    let event_type = event.event_type.clone();

                    combo_box = combo_box.on_input(move |d| {
                        let mut event_response = EventResponse::new(self_uid, event_type.clone());

                        event_response.data_str = Some(d);

                        Message::DomEvent(event_uid, event_response)
                    });
                }
                _ => (),
            }
        }

        // Register any events here
        // for event in events {
        //     match event.event_type.as_str() {
        //         "click" => {
        //             button = button.on_press(Message::DomEvent(
        //                 event.event_uid,
        //                 EventResponse::new(self_uid),
        //             ));
        //         }
        //         _ => (),
        //     }
        // }

        return combo_box.into();
    }

    fn process_event(&mut self, event: &DomInternalMessageType) -> Option<ElementEventResponse> {
        match event {
            DomInternalMessageType::PropertyChange(key, value) => match key.as_str() {
                "placeholder" => {
                    self.placeholder = value.clone();

                    return Some(ElementEventResponse::success());
                }
                _ => None,
            },
            DomInternalMessageType::GetProperty(key) => match key.as_str() {
                "placeholder" => {
                    return Some(ElementEventResponse::new(
                        QueryResponse::success().with_data_str(self.placeholder.clone()),
                    ));
                }
                "value" => {
                    if let Some(selected) = &self.selected {
                        return Some(ElementEventResponse::new(
                            QueryResponse::success().with_data_str(selected.id.clone()),
                        ));
                    } else {
                        return Some(ElementEventResponse::success());
                    }
                }
                _ => None,
            },
            DomInternalMessageType::FireEvent(event) => match event {
                CustomElementEvent::AddSelectOption(key, value) => {
                    let entry = SelectEntry {
                        text: value.clone(),
                        id: key.clone(),
                    };

                    self.state.push(entry);

                    return Some(ElementEventResponse::success());
                }
                // We unfortunately cannot remove an option, so we need to recreate the state with the remaining options
                CustomElementEvent::RemoveSelectOption(key) => {
                    let options = self.state.options();
                    let new_options: Vec<SelectEntry> = options
                        .iter()
                        .filter(|entry| entry.id != *key)
                        .cloned()
                        .collect();

                    // If the selected option is the one being removed, we need to clear the selection
                    if self.selected.is_some() && self.selected.as_ref().unwrap().id == *key {
                        self.selected = None;
                    }
                    self.state = iced::widget::combo_box::State::with_selection(
                        new_options,
                        self.selected.as_ref(),
                    );

                    return Some(ElementEventResponse::success());
                }
                CustomElementEvent::ClearSelectOptions() => {
                    self.state = iced::widget::combo_box::State::with_selection(Vec::new(), None);
                    self.selected = None;
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
        event_response: &EventResponse,
    ) -> Option<ElementEventResponse> {
        match event_type {
            EventListenerTypes::Select => {
                if let Some(id) = event_response.data_str.clone() {
                    let correct_entry = self.state.options().iter().find(|entry| entry.id == id);

                    if let Some(correct_entry) = correct_entry {
                        self.selected = Some(correct_entry.clone());
                    }
                }
                None
            }
            _ => None,
        }
    }
}
