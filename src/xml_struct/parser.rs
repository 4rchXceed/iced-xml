use std::{collections::HashMap, io::Cursor};

use quick_xml::{
    Reader,
    events::{BytesStart, Event, attributes::Attribute},
};

use crate::{
    css::parser::CssReader,
    xml_struct::theming::{Fonts, XmlTheme, gen_styles_log},
};

use super::xml_element::XmlElement;
use log::error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyValueProcessError {
    #[error("Failed to convert key to UTF-8 string: {0}")]
    KeyConversionError(String),
}

#[derive(Debug, Error)]
pub enum XmlElementCreationError {
    #[error("Failed to convert tag to UTF-8 string: {0}")]
    FailedToReadTag(String),
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
    pub fn new(reader: &mut Reader<Cursor<Vec<u8>>>, fonts: &Fonts) -> Result<Self, String> {
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut stack: Vec<XmlElement> = Vec::new();
        let mut root: Option<XmlElement> = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => {
                    return Err(e.to_string());
                }
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    let new_element = Self::new_element(e, fonts).map_err(|e| e.to_string())?;
                    stack.push(new_element);
                }
                Ok(Event::Text(e)) => {
                    if let Some(top) = stack.last_mut()
                        && let Ok(text) = e.decode()
                    {
                        top.text.push_str(text.into_owned().as_str());
                    }
                }
                Ok(Event::Empty(e)) => {
                    let new_element = Self::new_element(e, fonts).map_err(|e| e.to_string())?;
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(Box::new(new_element));
                    } else {
                        root = Some(new_element);
                    }
                }
                Ok(Event::End(_)) => {
                    if let Some(node) = stack.pop() {
                        if let Some(parent) = stack.last_mut() {
                            parent.children.push(Box::new(node));
                        } else {
                            root = Some(node);
                        }
                    }
                }
                _ => {}
            }
        }

        match root {
            Some(root_element) => Ok(XmlParser {
                root: root_element,
                css_parser: CssReader::new(""),
            }),
            None => Err("No root element found in the XML document.".to_string()),
        }
    }

    fn process_attribute(
        attr: Attribute<'_>,
        theme: &mut XmlTheme,
        fonts: &Fonts,
        id: &mut Option<String>,
        classes_string: &mut String,
        datas: &mut HashMap<String, String>,
    ) -> Result<(String, String), KeyValueProcessError> {
        let key = String::from_utf8(attr.key.as_ref().to_vec())
            .map_err(|e| KeyValueProcessError::KeyConversionError(e.to_string()))?;
        let value = String::from_utf8(attr.value.to_vec()).unwrap_or_default(); // default: ""

        if let Some(stripped_key) = key.strip_prefix("style:") {
            gen_styles_log(&stripped_key.to_string(), &value, theme, fonts);
        }

        if key == "id" {
            *id = Some(value.clone());
        } else if key == "classes" {
            *classes_string = value.clone();
        } else if let Some(stripped_key) = key.strip_prefix("data-") {
            datas.insert(stripped_key.to_string(), value.clone());
        }

        return Ok((key, value));
    }

    /// Utility function to create a new XmlElement from a BytesStart event.
    /// Used by the XmlParser to create XmlElements from the XML document.
    fn new_element(
        e: BytesStart<'_>,
        fonts: &Fonts,
    ) -> Result<XmlElement, XmlElementCreationError> {
        // last_theme = last_theme.clone();
        let mut id: Option<String> = None;
        let mut classes_string: String = String::new();
        let mut datas: HashMap<String, String> = HashMap::new();

        let mut theme = XmlTheme::default();

        let attributes: HashMap<String, String> = e
            .attributes()
            .filter_map(|a| {
                if let Ok(attr) = a {
                    match Self::process_attribute(
                        attr,
                        &mut theme,
                        fonts,
                        &mut id,
                        &mut classes_string,
                        &mut datas,
                    ) {
                        Ok((k, v)) => Some((k, v)),
                        Err(e) => {
                            error!("Error processing attribute: {}", e);
                            None
                        }
                    }
                } else {
                    return None;
                }
            })
            .collect();

        let classes: Vec<String> = classes_string
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|v| v.to_string())
            .collect();

        let tag = String::from_utf8(e.name().as_ref().to_vec())
            .map_err(|e| XmlElementCreationError::FailedToReadTag(e.to_string()))?;

        return Ok(XmlElement {
            tag: tag,
            attributes: attributes,
            children: Vec::new(),
            text: String::new(),
            theme: theme.clone(),
            id: id,
            classes: classes,
            datas: datas,
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

    match parser {
        Ok(p) => p.root,
        Err(e) => {
            panic!(
                "Failed to parse XML content: {}. Please use try_xml if you want to handle errors gracefully.",
                e
            );
        }
    }
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

    let parser = XmlParser::new(&mut reader, &fonts)
        .map_err(|e| format!("Failed to parse XML content: {}", e))?;
    return Ok(parser.root);
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
