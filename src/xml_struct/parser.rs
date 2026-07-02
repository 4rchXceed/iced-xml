use std::{collections::HashMap, io::Cursor};

use quick_xml::{Reader, events::Event};

use crate::{
    css_reader::CssReader,
    xml_struct::theming::{XmlTheme, gen_styles},
};

#[derive(Debug, Clone)]
pub enum XmlChangeEvent {
    StyleChange(String, String),    // k => v
    PropertyChange(String, String), // k => v
    GetProperty(String),            // key
    EventFired(String),             // event name
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
}

pub struct XmlParser {
    pub root: XmlElement,
    pub css_parser: CssReader,
}

impl XmlParser {
    pub fn new(reader: &mut Reader<Cursor<Vec<u8>>>) -> Self {
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        let mut stack: Vec<XmlElement> = Vec::new();
        let mut root: Option<XmlElement> = None;
        let mut last_theme: XmlTheme = XmlTheme::default();

        loop {
            match reader.read_event_into(&mut buf) {
                Err(_) => {
                    panic!("Failed to read XML File");
                }
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => match e.name().as_ref() {
                    _ => {
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
                                gen_styles(&k, &v, &mut last_theme);
                                if k == "id" {
                                    id = Some(v.clone());
                                }
                                if k == "classes" {
                                    classes_string = v.clone();
                                }
                                if k.starts_with("data-") {
                                    datas.insert(
                                        k.strip_prefix("data-").unwrap().to_string(),
                                        v.clone(),
                                    );
                                }
                                (k, v)
                            })
                            .collect::<Vec<_>>();
                        let attributes: HashMap<String, String> =
                            attributes.into_iter().map(|(k, v)| (k, v)).collect();
                        let classes: Vec<String> = classes_string
                            .split(" ")
                            .collect::<Vec<&str>>()
                            .iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>();
                        stack.push(XmlElement {
                            tag: String::from_utf8(e.name().as_ref().to_vec()).unwrap(),
                            attributes: attributes,
                            children: Vec::new(),
                            text: String::new(),
                            theme: last_theme.clone(),
                            id: id,
                            classes: classes,
                            datas: datas,
                        });
                    }
                },
                Ok(Event::Text(e)) => {
                    if let Some(top) = stack.last_mut() {
                        top.text.push_str(e.decode().unwrap().into_owned().as_str());
                    }
                }
                Ok(Event::End(_)) => {
                    let node = stack.pop().unwrap();

                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(node);
                    } else {
                        let theme = node.theme.clone();
                        root = Some(node);
                        last_theme = theme;
                    }
                }
                _ => {}
            }
        }
        return Self {
            root: root.unwrap().clone(),
            css_parser: CssReader::new(""),
        };
    }
}
