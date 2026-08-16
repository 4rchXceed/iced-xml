use std::fmt;

use iced::{Border, Shadow};

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
    fn new(xml_element: &XmlElement, _: &mut ElementRenderer, _: i32) -> Self {
        // If it supports children, initialize them here with renderer.init_element
        let mut options: Vec<SelectEntry> = Vec::new();
        let mut selected_id: Option<usize> = None;
        if !xml_element.text.trim().is_empty() {
            println!(
                "Warning: <Select /> element has text content, but it will be ignored. Only <Option /> children are allowed"
            );
        }
        for child in &xml_element.children {
            if child.tag == "Option" {
                let value_op = child.attributes.get("value");
                let mut value = child.text.trim();
                if value_op.is_some() {
                    value = value_op.unwrap();
                } else {
                    println!(
                        "Warning <Option /> element doesn't have a value=\"\" attribute, using text as id, but it's not recommended"
                    );
                }
                let entry = SelectEntry {
                    text: child.text.trim().to_string(),
                    id: value.to_string(),
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
                panic!(
                    "Only <Option selected=\"true|false\">text</Option> are allowed to be children of ComboBox, not <{} />",
                    child.tag
                );
            }
        }

        let placeholder_op = xml_element.attributes.get("placeholder");
        let mut placeholder: String = "Please choose an option".to_string();
        if placeholder_op.is_some() {
            placeholder = placeholder_op.unwrap().clone();
        } else {
            println!("Using auto-generated placeholder for <Select /> is not recommended");
        }

        let mut selected: Option<&SelectEntry> = None;

        if selected_id.is_some() {
            selected = options.get(selected_id.unwrap());
        }

        Self {
            state: iced::widget::combo_box::State::with_selection(options.clone(), selected),
            placeholder: placeholder,
            selected: selected.cloned(),
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
        let mut select_event_uid: Option<i32> = None;
        for event in &events {
            match event.event_type.as_str() {
                "selected" => {
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
                    let mut event_response = EventResponse::new(self_uid, "selected".to_string());
                    event_response.data_str = Some(selected_entry.id);
                    if select_event_uid.is_some() {
                        return Message::DomEvent(select_event_uid.unwrap(), event_response);
                    } else {
                        return Message::DomEvent(-1, event_response);
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

        if theme.size.is_some() {
            combo_box = combo_box.size(theme.size.unwrap());
        }

        if datas.flag_themes.get("input").is_some() {
            let theme = datas.flag_themes.get("input").cloned().unwrap();
            combo_box = combo_box.input_style(move |_, _| iced::widget::text_input::Style {
                background: iced::Background::Color(theme.background_color), // TODO: Add support for gradient backgrounds
                border: Border {
                    color: theme.border_color,
                    radius: theme.border_radius,
                    width: theme.border_width,
                },
                icon: theme.icon_color,
                placeholder: theme.input_placeholder_color,
                value: theme.foreground_color,
                selection: theme.selection_color,
            });
        }

        if datas.flag_themes.get("select-menu").is_some() {
            let theme = datas.flag_themes.get("select-menu").cloned().unwrap();
            combo_box = combo_box.menu_style(move |_| iced::overlay::menu::Style {
                background: iced::Background::Color(theme.background_color), // TODO: Add support for gradient backgrounds
                border: Border {
                    color: theme.border_color,
                    radius: theme.border_radius,
                    width: theme.border_width,
                },
                text_color: theme.foreground_color,
                selected_background: iced::Background::Color(theme.selected_background_color),
                selected_text_color: theme.selected_text_color,
                shadow: Shadow {
                    color: theme.shadow_color,
                    offset: theme.shadow_offset,
                    blur_radius: theme.shadow_blur_radius,
                },
            });
        }

        if theme.select_icon.is_some() {
            combo_box = combo_box.icon(theme.select_icon.unwrap());
        }

        for event in events {
            match event.event_type.as_str() {
                "onclose" => {
                    combo_box = combo_box.on_close(Message::DomEvent(
                        event.event_uid,
                        EventResponse::new(self_uid, event.event_type.clone()),
                    ));
                }
                "onopen" => {
                    combo_box = combo_box.on_open(Message::DomEvent(
                        event.event_uid,
                        EventResponse::new(self_uid, event.event_type.clone()),
                    ));
                }
                "oninput" => {
                    let event_uid = event.event_uid;
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

    fn process_event(
        &mut self,
        event: &XmlChangeEvent,
    ) -> Option<(QueryResponse, Vec<i32>, Vec<RendererEvent>)> {
        match event {
            // Process PropertyChange / GetProperty / Custom events
            // The second parameter is a list of element IDs to forward the event to
            // For example, if you want to forward the event to your "virtual label element", so it changes text,
            // you would use `Some(..., vec![virtual_text])`
            XmlChangeEvent::EventFired(event_name, response) => {
                if event_name == "selected" {
                    let id_op = response.data_str.clone();
                    if id_op.is_some() {
                        let id = id_op.unwrap();
                        let correct_entry =
                            self.state.options().iter().find(|entry| entry.id == id);
                        if correct_entry.is_some() {
                            self.selected = Some(correct_entry.unwrap().clone());
                        }
                    }
                    None
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
