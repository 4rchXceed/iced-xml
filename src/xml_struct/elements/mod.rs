use std::{collections::HashMap};

use iced::widget::text;

use crate::{Message, logger::fatal, xml_struct::{elements::{container::Container, element_base::ElementBase, label::Label, button::Button}, parser::XmlElement}};

pub mod element_base;
pub mod label;
pub mod container;
pub mod button;

pub enum AnyElement {
    Label(Label),
    Container(Container),
    Button(Button),
}

pub struct ElementRenderer {
    pub elements: HashMap<i32, AnyElement>,
    pub last_uid: i32,
}

impl ElementRenderer {
    pub fn new() -> Self {
        Self { elements: HashMap::new(), last_uid: 0 }
    }

    pub fn init_element(&mut self, xml_element: &XmlElement) -> i32 {
        let element: Option<AnyElement> = match xml_element.tag.as_str() {
            "Label" => Some(AnyElement::Label(Label::new(xml_element, self))),
            "Div" => Some(AnyElement::Container(Container::new(xml_element, self))),
            "Window" => Some(AnyElement::Container(Container::new(xml_element, self))), // Window works the same way as a container (FOR NOW), we'll use the same logic
            "Button" => Some(AnyElement::Button(Button::new(xml_element, self))),
            _ => None
        };
        if let Some(element) = element {
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
            let output = match element.unwrap() {
                AnyElement::Label(label) => label.render(self),
                AnyElement::Container(container) => container.render(self),
                AnyElement::Button(button) => button.render(self)
            };
            output
        } else {
            return text(format!("Element with id {} not found", uid)).into();
        }
    }
}
