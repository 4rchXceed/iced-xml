use std::{collections::HashMap, io::Cursor};

use quick_xml::{
    Error, Reader,
    events::{BytesStart, Event},
};

use crate::{
    css_reader::CssReader,
    xml_struct::theming::{Fonts, XmlTheme, gen_styles},
};

/// This struct represents an XML element.
/// See the fields for more information.
#[derive(Debug, Clone)]
pub struct XmlElement {
    /// The element tag name.
    /// Example: Div for a <Div> element.
    pub tag: String,
    /// The element attributes as a HashMap.
    /// Example: "id" => "my-id for <Div id="my-id">.
    pub attributes: HashMap<String, String>,
    /// The element text content.
    /// Example: "Hello World" for <Div>Hello World</Div>.
    pub text: String,
    /// The element children as a Vec of XmlElement.
    /// Example: [XmlElement { tag: "Span", text: "Hello" }] for <Div><Span>Hello</Span></Div>.
    pub children: Vec<XmlElement>,
    /// The element theme as an XmlTheme.
    /// View XmlTheme for more information.
    pub theme: XmlTheme,
    /// The element id as an Option<String>.
    /// Example: Some("my-id") for <Div id="my-id">.
    pub id: Option<String>,
    /// The element classes as a Vec<String>.
    /// Example: ["class1", "class2"] for <Div class="class1 class2">.
    pub classes: Vec<String>,
    /// The element data attributes as a HashMap.
    /// Example: "key" => "value" for <Div data-key="value">.
    pub datas: HashMap<String, String>,
}

impl XmlElement {
    /// Creates a virtual XmlElement
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

    /// Creates an empty XmlElement, that will be of size 0x0
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

    /// Sets the tag of the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// element.set_tag("Div");
    /// ```
    pub fn set_tag(&mut self, tag: &str) {
        self.tag = String::from(tag);
    }

    /// Sets the text content of the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// element.set_text("Hello World");
    /// ```
    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
    }

    /// Sets the attributes of the XmlElement.
    /// Examples:
    /// ```rust
    /// use std::collections::HashMap;
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// let mut attributes = HashMap::new();
    /// attributes.insert(String::from("id"), String::from("my-id"));
    /// element.set_attributes(attributes);
    /// ```
    pub fn set_attributes(&mut self, attributes: HashMap<String, String>) {
        self.attributes = attributes;
    }

    /// Sets the children of the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// let child = iced_xml::xml_struct::parser::XmlElement::void();
    /// element.set_children(vec![child]);
    /// ```
    pub fn set_children(&mut self, children: Vec<XmlElement>) {
        self.children = children;
    }

    /// Adds a rule the theme of the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// element.apply_css("bg", "red");
    /// ```
    pub fn apply_css(&mut self, key: &str, value: &str) {
        gen_styles(
            &String::from(key),
            &String::from(value),
            &mut self.theme,
            &Fonts::default(),
        );
    }

    /// Appends a child to the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// let child = iced_xml::xml_struct::parser::XmlElement::void();
    /// element.append_child(child);
    /// ```
    pub fn append_child(&mut self, child: XmlElement) {
        self.children.push(child);
    }

    /// Removes a child from the XmlElement by index.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// let child = iced_xml::xml_struct::parser::XmlElement::void();
    /// element.append_child(child);
    /// element.remove_child(0);
    /// ```
    pub fn remove_child(&mut self, index: usize) {
        if index < self.children.len() {
            self.children.remove(index);
        }
    }

    /// Returns the number of children of the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// let child = iced_xml::xml_struct::parser::XmlElement::void();
    /// element.append_child(child);
    /// assert_eq!(element.children_count(), 1);
    /// ```
    pub fn children_count(&self) -> usize {
        self.children.len()
    }

    /// Sets the id of the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// element.set_id("my-id");
    /// ```
    pub fn set_id(&mut self, id: &str) {
        self.id = Some(String::from(id));
    }

    /// Clears the id of the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// element.set_id("my-id");
    /// element.clear_id();
    /// ```
    pub fn clear_id(&mut self) {
        self.id = None;
    }

    /// Adds a class to the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// element.add_class("my-class");
    /// ```
    pub fn add_class(&mut self, class: &str) {
        if !self.classes.contains(&String::from(class)) {
            self.classes.push(String::from(class));
        }
    }

    /// Removes a class from the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// element.add_class("my-class");
    /// element.remove_class("my-class");
    /// ```
    pub fn remove_class(&mut self, class: &str) {
        self.classes.retain(|c| c != class);
    }

    /// Adds a data attribute to the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// element.add_data("key", "value");
    /// ```
    pub fn add_data(&mut self, key: &str, value: &str) {
        self.datas.insert(String::from(key), String::from(value));
    }

    /// Removes a data attribute from the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// element.add_data("key", "value");
    /// element.remove_data("key");
    /// ```
    pub fn remove_data(&mut self, key: &str) {
        self.datas.remove(key);
    }
}

impl std::fmt::Display for XmlElement {
    /// Pretty prints the XmlElement as a string.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::parser::XmlElement::void();
    /// element.set_tag("Div");
    /// element.set_text("Hello World");
    /// println!("{}", element);
    /// ```
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

/// Utility function to create a new XmlElement from a BytesStart event.
/// Used by the XmlParser to create XmlElements from the XML document.
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
