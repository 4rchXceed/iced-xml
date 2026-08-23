use std::{collections::HashMap, io::Cursor};

use quick_xml::{
    Error, Reader,
    events::{BytesStart, Event},
};

use crate::{
    css_reader::CssReader,
    dom::query::{DomEvent, EventResponse},
    xml_struct::theming::{Fonts, XmlTheme, gen_styles},
};

#[derive(Debug, Clone)]
pub enum XmlChangeEvent {
    StyleChange(String, String, Option<String>), // k => v, flag
    PropertyChange(String, String),              // k => v
    GetProperty(String),                         // key
    EventFired(String, EventResponse),           // event name
    EmittedEvent(String, DomEvent),              // event name, event data
}

#[derive(Debug, Clone)]
pub struct XmlElement {
    pub tag: String,
    pub attributes: HashMap<String, String>,
    pub text: String,
    pub children: Vec<XmlElement>,
    pub theme: XmlTheme,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub datas: HashMap<String, String>,
}

impl XmlElement {
    pub fn virt() -> Self {
        Self {
            tag: String::from("$virtual"),
            attributes: HashMap::new(),
            text: String::new(),
            children: Vec::new(),
            theme: XmlTheme::default(),
            id: None,
            classes: Vec::new(),
            datas: HashMap::new(),
        }
    }
    pub fn void() -> Self {
        Self {
            tag: String::from("Void"), // Use the void element
            attributes: HashMap::new(),
            text: String::new(),
            children: Vec::new(),
            theme: XmlTheme::default(),
            id: None,
            classes: Vec::new(),
            datas: HashMap::new(),
        }
    }
    pub fn set_tag(&mut self, tag: &str) {
        self.tag = String::from(tag);
    }
    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
    }
    pub fn set_attributes(&mut self, attributes: HashMap<String, String>) {
        self.attributes = attributes;
    }
    pub fn set_children(&mut self, children: Vec<XmlElement>) {
        self.children = children;
    }
    pub fn apply_css(&mut self, key: &str, value: &str) {
        gen_styles(
            &String::from(key),
            &String::from(value),
            &mut self.theme,
            &Fonts::default(),
        );
    }
    pub fn append_child(&mut self, child: XmlElement) {
        self.children.push(child);
    }
    pub fn remove_child(&mut self, index: usize) {
        if index < self.children.len() {
            self.children.remove(index);
        }
    }
    pub fn children_count(&self) -> usize {
        self.children.len()
    }
    pub fn set_id(&mut self, id: &str) {
        self.id = Some(String::from(id));
    }
    pub fn clear_id(&mut self) {
        self.id = None;
    }
    pub fn add_class(&mut self, class: &str) {
        if !self.classes.contains(&String::from(class)) {
            self.classes.push(String::from(class));
        }
    }
    pub fn remove_class(&mut self, class: &str) {
        self.classes.retain(|c| c != class);
    }
    pub fn add_data(&mut self, key: &str, value: &str) {
        self.datas.insert(String::from(key), String::from(value));
    }
    pub fn remove_data(&mut self, key: &str) {
        self.datas.remove(key);
    }
}

impl std::fmt::Display for XmlElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attributes_string = self
            .attributes
            .iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, v))
            .collect::<Vec<String>>()
            .join(" ");
        let children_string = self
            .children
            .iter()
            .map(|child| format!("{}", child))
            .collect::<Vec<String>>()
            .join("");
        write!(
            f,
            "<{} {}>{}{}</{}>",
            self.tag, attributes_string, self.text, children_string, self.tag
        )
    }
}

fn new_element(last_theme: &mut XmlTheme, e: BytesStart<'_>, fonts: &Fonts) -> XmlElement {
    // last_theme = last_theme.clone();
    let mut id: Option<String> = None;
    let mut classes_string: String = String::new();
    let mut datas: HashMap<String, String> = HashMap::new();

    let attributes = e
        .attributes()
        .map(|a| {
            let b = a.unwrap();
            let k = String::from_utf8(b.key.as_ref().to_vec()).unwrap();
            let v = String::from_utf8(b.value.to_vec()).unwrap();
            if k.starts_with("style:") {
                gen_styles(
                    &k.strip_prefix("style:").unwrap().to_string(),
                    &v,
                    last_theme,
                    fonts,
                );
            }
            if k == "id" {
                id = Some(v.clone());
            }
            if k == "classes" {
                classes_string = v.clone();
            }
            if k.starts_with("data-") {
                datas.insert(k.strip_prefix("data-").unwrap().to_string(), v.clone());
            }
            (k, v)
        })
        .collect::<Vec<_>>();
    let attributes: HashMap<String, String> = attributes.into_iter().map(|(k, v)| (k, v)).collect();
    let classes: Vec<String> = classes_string
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    return XmlElement {
        tag: String::from_utf8(e.name().as_ref().to_vec()).unwrap(),
        attributes: attributes,
        children: Vec::new(),
        text: String::new(),
        theme: last_theme.clone(),
        id: id,
        classes: classes,
        datas: datas,
    };
}

pub struct XmlParser {
    pub root: XmlElement,
    pub css_parser: CssReader,
}

impl XmlParser {
    pub fn new(reader: &mut Reader<Cursor<Vec<u8>>>, fonts: &Fonts) -> Result<Self, Error> {
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        let mut stack: Vec<XmlElement> = Vec::new();
        let mut root: Option<XmlElement> = None;
        let mut last_theme: XmlTheme = XmlTheme::default();

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => {
                    return Err(e);
                }
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    let new_element = new_element(&mut last_theme, e, fonts);
                    stack.push(new_element);
                }
                Ok(Event::Text(e)) => {
                    if let Some(top) = stack.last_mut() {
                        top.text.push_str(e.decode().unwrap().into_owned().as_str());
                    }
                }
                Ok(Event::Empty(e)) => {
                    let new_element = new_element(&mut last_theme, e, fonts);
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(new_element);
                    } else {
                        root = Some(new_element);
                    }
                }
                Ok(Event::End(_)) => {
                    let node = stack.pop().unwrap();

                    let theme = node.theme.clone();
                    last_theme = theme;

                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(node);
                    } else {
                        root = Some(node);
                    }
                }
                _ => {}
            }
        }
        return Ok(Self {
            root: root.unwrap().clone(),
            css_parser: CssReader::new(""),
        });
    }
}

pub fn xml_fonts(xml_string: &str, fonts: Fonts) -> XmlElement {
    let mut reader = Reader::from_reader(Cursor::new(xml_string.as_bytes().to_vec()));
    let parser = XmlParser::new(&mut reader, &fonts);
    if parser.is_err() {
        panic!(
            "Failed to parse XML content: {:?}. Please use try_xml if you want to handle errors gracefully.",
            parser.err()
        );
    }
    return parser.unwrap().root;
}

pub fn try_xml_fonts(xml_string: &str, fonts: Fonts) -> Result<XmlElement, String> {
    let mut reader = Reader::from_reader(Cursor::new(xml_string.as_bytes().to_vec()));
    let parser = XmlParser::new(&mut reader, &fonts);
    if parser.is_err() {
        return Err(format!("Failed to parse XML content: {:?}", parser.err()));
    } else {
        return Ok(parser.unwrap().root);
    }
}

pub fn xml(xml_string: &str) -> XmlElement {
    let fonts = Fonts::default();
    return xml_fonts(xml_string, fonts);
}

pub fn try_xml(xml_string: &str) -> Result<XmlElement, String> {
    let fonts = Fonts::default();
    return try_xml_fonts(xml_string, fonts);
}
