use std::{collections::HashMap, io::Cursor};

use iced::{
    Color, Length, Padding, Vector,
    alignment::{Horizontal, Vertical},
    border::Radius,
    widget::button::DEFAULT_PADDING,
};
use quick_xml::{Reader, events::Event};

use crate::{
    css_reader::CssReader,
    logger::fatal,
    utilsfn::{
        parse_align_x, parse_align_y, parse_color, parse_length, parse_padding, parse_radius,
        parse_value, parse_vector,
    },
};

pub enum XmlChangeEvent {
    StyleChange(String, String),    // k => v
    PropertyChange(String, String), // k => v
    GetProperty(String),            // key
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
    pub datas: HashMap<String, String>,
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
    pub clip: bool,
    pub height: Length,
    pub width: Length,
    pub padding: Padding,
    pub spacing: f32,
    pub max_width: f32,
    pub align_x: Horizontal,
    pub align_y: Vertical,
    pub wrap: bool,
}

impl XmlTheme {
    /**
     * Changes the values of the current theme to match the values of `changes` only on the properties that are different between `first` and `second`.
     */
    pub fn apply_only_changes(&mut self, first: &XmlTheme, second: &XmlTheme, changes: &XmlTheme) {
        if first.background_color != second.background_color {
            self.background_color = changes.background_color;
        }
        if first.text_color != second.text_color {
            self.text_color = changes.text_color;
        }
        if first.snap != second.snap {
            self.snap = changes.snap;
        }
        if first.shadow_color != second.shadow_color {
            self.shadow_color = changes.shadow_color;
        }
        if first.shadow_blur_radius != second.shadow_blur_radius {
            self.shadow_blur_radius = changes.shadow_blur_radius;
        }
        if first.shadow_offset != second.shadow_offset {
            self.shadow_offset = changes.shadow_offset;
        }
        if first.border_color != second.border_color {
            self.border_color = changes.border_color;
        }
        if first.border_radius != second.border_radius {
            self.border_radius = changes.border_radius;
        }
        if first.border_width != second.border_width {
            self.border_width = changes.border_width;
        }
        if first.clip != second.clip {
            self.clip = changes.clip;
        }
        if first.height != second.height {
            self.height = changes.height;
        }
        if first.width != second.width {
            self.width = changes.width;
        }
        if first.padding != second.padding {
            self.padding = changes.padding;
        }
        if first.spacing != second.spacing {
            self.spacing = changes.spacing;
        }
        if first.max_width != second.max_width {
            self.max_width = changes.max_width;
        }
        if first.align_x != second.align_x {
            self.align_x = changes.align_x;
        }
        if first.align_y != second.align_y {
            self.align_y = changes.align_y;
        }
        if first.wrap != second.wrap {
            self.wrap = changes.wrap;
        }
    }
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
            clip: false,
            height: Length::Shrink,
            width: Length::Shrink,
            padding: DEFAULT_PADDING,
            max_width: f32::INFINITY,
            spacing: 0.0,
            align_x: Horizontal::Left,
            align_y: Vertical::Top,
            wrap: false,
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
    pub css_parser: CssReader,
}

pub fn gen_styles(key: &String, value: &String, theme: &mut XmlTheme) {
    match key.as_str() {
        "bg" => theme.background_color = parse_color(value),
        "fg" => theme.text_color = parse_color(value),
        "snap" => theme.snap = value == "true",
        "shadow-color" => theme.shadow_color = parse_color(value),
        "shadow-blur" => theme.shadow_blur_radius = value.parse().unwrap_or_default(),
        "shadow-offset" => theme.shadow_offset = parse_vector(value),
        "border-color" => theme.border_color = parse_color(value),
        "border-radius" => theme.border_radius = parse_radius(value),
        "border-width" => theme.border_width = value.parse().unwrap_or_default(),
        "clip" => theme.clip = value == "true",
        "height" => theme.height = parse_length(value),
        "width" => theme.width = parse_length(value),
        "padding" => theme.padding = parse_padding(value),
        "max_width" => theme.max_width = value.parse().unwrap_or_default(),
        "align-x" => theme.align_x = parse_align_x(value),
        "spacing" => theme.spacing = parse_value(value),
        "align-y" => theme.align_y = parse_align_y(value),
        "wrap" => theme.wrap = value == "true",
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
