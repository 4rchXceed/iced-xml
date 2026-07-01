use std::collections::HashMap;

use iced::widget::text;

use crate::{
    css_reader::CssReader,
    dom::events::DomQuery,
    logger::fatal,
    xml_engine::Message,
    xml_struct::{
        elements::{button::Button, container::Container, element_base::ElementBase, label::Label},
        parser::{XmlChangeEvent, XmlElement},
    },
};

pub mod button;
pub mod container;
pub mod element_base;
pub mod label;

pub enum AnyElement {
    Label(Label),
    Container(Container),
    Button(Button),
}

pub struct EventListener {
    pub event_type: String,
    pub target: i32,
    pub handler: i32,
    pub event_uid: i32,
}

pub struct ElementRenderer {
    pub elements: HashMap<i32, AnyElement>,
    pub id_map: HashMap<String, i32>,
    pub classes_map: HashMap<String, Vec<i32>>,
    pub last_uid: i32,
    pub event_listeners: Vec<EventListener>,
}

impl ElementRenderer {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            last_uid: 0,
            id_map: HashMap::new(),
            classes_map: HashMap::new(),
            event_listeners: Vec::new(),
        }
    }

    pub fn load_css(&mut self, css: &str) {
        let mut reader = CssReader::new(css);
        reader.parse();
        for rule_block in reader.rules {
            // TODO: Add support for multiple selectors in a single rule block
            let selector = rule_block.selector;
            let dom_query = DomQuery::new(selector.selector_type, selector.content);

            let elements = self.element_query(&dom_query);
            if elements.is_some() {
                for element in elements.unwrap() {
                    for rule in rule_block.rules.iter() {
                        self.emit_internal_event(
                            element,
                            XmlChangeEvent::StyleChange(rule.name.clone(), rule.value.clone()),
                        );
                    }
                }
            }
        }
    }

    pub fn element_query(&self, query: &DomQuery) -> Option<Vec<i32>> {
        return match query {
            DomQuery::ById(id) => {
                if let Some(uid) = self.id_map.get(id) {
                    Some(vec![*uid])
                } else {
                    None
                }
            }
            DomQuery::ByUid(uid) => {
                if self.elements.contains_key(&uid) {
                    Some(vec![uid.clone()])
                } else {
                    None
                }
            } // _ => None,
            DomQuery::Class(class) => {
                if let Some(uids) = self.classes_map.get(class) {
                    Some(uids.clone())
                } else {
                    None
                }
            }
            DomQuery::All => Some(self.elements.keys().cloned().collect()),
            DomQuery::Unused => None,
        };
    }

    pub fn init_element(&mut self, xml_element: &XmlElement) -> i32 {
        // TODO: Add "plugin" support (function provided by the user to resolve custom elements)
        let element: Option<AnyElement> = match xml_element.tag.as_str() {
            "Label" => Some(AnyElement::Label(Label::new(xml_element, self))),
            "Div" => Some(AnyElement::Container(Container::new(xml_element, self))),
            "Window" => Some(AnyElement::Container(Container::new(xml_element, self))), // Window works the same way as a container (FOR NOW), we'll use the same logic
            "Button" => Some(AnyElement::Button(Button::new(xml_element, self))),
            _ => None,
        };
        if let Some(element) = element {
            if xml_element.id.is_some() {
                self.id_map
                    .insert(xml_element.id.clone().unwrap().to_string(), self.last_uid);
            }
            for class in &xml_element.classes {
                let class_map = self.classes_map.get(class);
                if class_map.is_some() {
                    self.classes_map.get_mut(class).unwrap().push(self.last_uid);
                } else {
                    self.classes_map.insert(class.clone(), vec![self.last_uid]);
                }
            }
            self.elements.insert(self.last_uid, element);
            self.last_uid += 1;
            return self.last_uid - 1;
        } else {
            fatal(format!("Block: <{} /> doesn't exists", &xml_element.tag).as_str());
            return -1;
        }
    }

    pub fn render_element(&self, uid: i32) -> iced::Element<'_, Message> {
        let element = self.elements.get(&uid);
        if element.is_some() {
            let events = self
                .event_listeners
                .iter()
                .filter(|v| v.target == uid)
                .collect::<Vec<&EventListener>>();
            let output = match element.unwrap() {
                AnyElement::Label(label) => label.render(self, events),
                AnyElement::Container(container) => container.render(self, events),
                AnyElement::Button(button) => button.render(self, events),
            };
            output
        } else {
            return text(format!("Element with id {} not found", uid)).into();
        }
    }

    pub fn emit_internal_event(&mut self, uid: i32, event: XmlChangeEvent) {
        let element = self.elements.get_mut(&uid);
        if element.is_some() {
            match element.unwrap() {
                AnyElement::Label(label) => label.process_event(&event),
                AnyElement::Container(container) => container.process_event(&event),
                AnyElement::Button(button) => button.process_event(&event),
            }
        }
    }

    pub fn register_event(&mut self, event_type: String, target: i32, handler: i32) {
        self.event_listeners.push(EventListener {
            event_type: event_type,
            target: target,
            handler: handler,
            event_uid: self.last_uid,
        });
        self.last_uid += 1;
    }
}
