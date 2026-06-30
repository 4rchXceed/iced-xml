use std::io::Cursor;

use iced::{Color, Vector, border::Radius};
use quick_xml::{Reader, events::Event};

use crate::{
    logger::fatal,
    utilsfn::{parse_color, parse_radius, parse_vector},
};

pub enum XmlChangeEvent {
    StyleChange(String, String),    // k => v
    PropertyChange(String, String), // k => v
}

#[derive(Debug, Clone)]
pub struct XmlElement {
    pub tag: String,
    pub attributes: Vec<(String, String)>,
    pub text: String,
    pub children: Vec<XmlElement>,
    pub theme: XmlTheme,
    pub id: Option<String>,
    pub classes: Vec<String>,
}

// impl XmlElement {
//     fn new() -> Self {
//         Self {
//             tag: String::new(),
//             attributes: Vec::new(),
//             text: String::new(),
//             children: Vec::new(),
//             theme: XmlTheme::default(),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct XmlTheme {
    pub background_color: Color,
    pub text_color: Color,
    pub snap: bool,
    pub shadow_color: Color,
    pub shadow_blur_radius: f32,
    pub shadow_offset: Vector,
    pub border_color: Color,
    pub border_radius: Radius,
    pub border_width: f32,
}

impl Default for XmlTheme {
    fn default() -> Self {
        Self {
            background_color: Color::WHITE,
            text_color: Color::BLACK,
            snap: false,
            shadow_color: Color::BLACK,
            shadow_blur_radius: 0.0,
            shadow_offset: Vector { x: 0.0, y: 0.0 },
            border_color: Color::BLACK,
            border_radius: Radius::default(),
            border_width: 0.0,
        }
    }
}

// Not needed, but kept in case...
// impl std::fmt::Display for XmlElement {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "<{}", self.tag)?;
//         for (key, value) in &self.attributes {
//             write!(f, " {}=\"{}\"", key, value)?;
//         }
//         if self.children.is_empty() {
//             write!(f, " />")?;
//         } else {
//             write!(f, ">")?;
//             for child in self.children.iter() {
//                 write!(f, "{}", child)?;
//             }
//             write!(f, "</{}>", self.tag)?;
//         }
//         Ok(())
//     }
// }

pub struct XmlParser {
    pub root: XmlElement,
}

pub fn gen_styles(key: &String, value: &String, theme: &mut XmlTheme) {
    match key.as_str() {
        "bg" => theme.background_color = parse_color(value),
        "fg" => theme.text_color = parse_color(value),
        "snap" => theme.snap = value == "true",
        "shadow_color" => theme.shadow_color = parse_color(value),
        "shadow_blur" => theme.shadow_blur_radius = value.parse().unwrap_or_default(),
        "shadow_offset" => theme.shadow_offset = parse_vector(value),
        "border_color" => theme.border_color = parse_color(value),
        "border_radius" => theme.border_radius = parse_radius(value),
        "border_width" => theme.border_width = value.parse().unwrap_or_default(),
        _ => {}
    }
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
                    fatal("Failed to read XML File");
                }
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => match e.name().as_ref() {
                    _ => {
                        let mut theme = last_theme.clone();
                        let mut id: Option<String> = None;
                        let mut classes_string: String = String::new();
                        let attributes = e
                            .attributes()
                            .map(|a| {
                                let b = a.unwrap();
                                let k = String::from_utf8(b.key.as_ref().to_vec()).unwrap();
                                let v = String::from_utf8(b.value.to_vec()).unwrap();
                                gen_styles(&k, &v, &mut theme);
                                if k == "id" {
                                    id = Some(v.clone());
                                }
                                if k == "classes" {
                                    classes_string = v.clone();
                                }
                                (k, v)
                            })
                            .collect::<Vec<_>>();
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
                            theme: theme,
                            id: id,
                            classes: classes,
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
        };
    }
}
