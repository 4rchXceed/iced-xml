use std::{collections::HashMap, io::Cursor};

use quick_xml::{
    Error, Reader,
    events::{BytesStart, Event},
};

use crate::{
    css::parser::CssReader,
    xml_struct::theming::{Fonts, XmlTheme, gen_styles_log},
};

use super::xml_element::XmlElement;

/// Utility function to create a new XmlElement from a BytesStart event.
/// Used by the XmlParser to create XmlElements from the XML document.
fn new_element(e: BytesStart<'_>, fonts: &Fonts) -> XmlElement {
    // last_theme = last_theme.clone();
    let mut id: Option<String> = None;
    let mut classes_string: String = String::new();
    let mut datas: HashMap<String, String> = HashMap::new();
    let mut theme = XmlTheme::default();
    let attributes = e
        .attributes()
        .map(|a| {
            let b = a.unwrap();
            let k = String::from_utf8(b.key.as_ref().to_vec()).unwrap();
            let v = String::from_utf8(b.value.to_vec()).unwrap();
            if k.starts_with("style:") {
                gen_styles_log(
                    &k.strip_prefix("style:").unwrap().to_string(),
                    &v,
                    &mut theme,
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
        theme: theme.clone(),
        id: id,
        classes: classes,
        datas: datas,
    };
}

/// This struct represents an XML parser.
/// It uses the quick-xml crate to parse XML documents and create a tree of XmlElement structs.
pub struct XmlParser {
    pub root: XmlElement,
    pub css_parser: CssReader,
}

impl XmlParser {
    /// Creates a new XmlParser from a Reader and a Fonts struct.
    /// The Reader is used to read the XML document, and the Fonts struct is used to apply styles to the XmlElements.
    ///
    /// Parameters:
    /// - reader: The quick-xml Reader to read the XML document.
    /// - fonts: The Fonts struct to apply styles to the XmlElements.
    pub fn new(reader: &mut Reader<Cursor<Vec<u8>>>, fonts: &Fonts) -> Result<Self, Error> {
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        let mut stack: Vec<XmlElement> = Vec::new();
        let mut root: Option<XmlElement> = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => {
                    return Err(e);
                }
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    let new_element = new_element(e, fonts);
                    stack.push(new_element);
                }
                Ok(Event::Text(e)) => {
                    if let Some(top) = stack.last_mut() {
                        top.text.push_str(e.decode().unwrap().into_owned().as_str());
                    }
                }
                Ok(Event::Empty(e)) => {
                    let new_element = new_element(e, fonts);
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(Box::new(new_element));
                    } else {
                        root = Some(new_element);
                    }
                }
                Ok(Event::End(_)) => {
                    let node = stack.pop().unwrap();

                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(Box::new(node));
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

/// Utility function to parse an XML string into an XmlElement.
/// This function will panic if the XML string is invalid. Use try_xml if you want to handle errors gracefully.
///
/// This also supports a Fonts struct if you want to use custom fonts.
/// Please notice that if you change the fonts from a CSS file (import_css on the QueryBuilder), this is not needed.
/// The only time you need to use this, is if you want to use the style:font-family attribute
///
/// Example:
/// ```rust
/// let xml_string = r#"<Div style:font-family="CustomFont">Hello World</Div>"#;
/// let fonts = iced_xml::xml_struct::theming::Fonts::default();
/// let element = iced_xml::xml_struct::parser::xml_fonts(xml_string, fonts);
/// ```
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

/// Utility function to parse an XML string into an XmlElement.
/// This function will return a Result, allowing you to handle errors gracefully.
///
/// This also supports a Fonts struct if you want to use custom fonts.
/// Please notice that if you change the fonts from a CSS file (import_css on the QueryBuilder), this is not needed.
/// The only time you need to use this, is if you want to use the style:font-family attribute
///
/// Example:
/// ```rust
/// let xml_string = r#"<Div style:font-family="CustomFont">Hello World</Div>"#;
/// let fonts = iced_xml::xml_struct::theming::Fonts::default();
/// iced_xml::xml_struct::parser::try_xml_fonts(xml_string, fonts).expect("Failed to parse XML content");
/// ```
pub fn try_xml_fonts(xml_string: &str, fonts: Fonts) -> Result<XmlElement, String> {
    let mut reader = Reader::from_reader(Cursor::new(xml_string.as_bytes().to_vec()));
    let parser = XmlParser::new(&mut reader, &fonts);
    if parser.is_err() {
        return Err(format!("Failed to parse XML content: {:?}", parser.err()));
    } else {
        return Ok(parser.unwrap().root);
    }
}

/// Utility function to parse an XML string into an XmlElement.
/// This function will panic if the XML string is invalid. Use try_xml if you want to handle errors gracefully.
///
/// Example:
/// ```rust
/// let xml_string = r#"<Div>Hello World</Div>"#;
/// let element = iced_xml::xml_struct::parser::xml(xml_string);
/// ```
pub fn xml(xml_string: &str) -> XmlElement {
    let fonts = Fonts::default();
    return xml_fonts(xml_string, fonts);
}

/// Utility function to parse an XML string into an XmlElement.
/// This function will return a Result, allowing you to handle errors gracefully.
///
/// Example:
/// ```rust
/// let xml_string = r#"<Div>Hello World</Div>"#;
/// iced_xml::xml_struct::parser::try_xml(xml_string).expect("Failed to parse XML content");
/// ```
pub fn try_xml(xml_string: &str) -> Result<XmlElement, String> {
    let fonts = Fonts::default();
    return try_xml_fonts(xml_string, fonts);
}
