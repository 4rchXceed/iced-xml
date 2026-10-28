use std::collections::HashMap;

use super::theming::{Fonts, XmlTheme, gen_styles_log};

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
    pub children: Vec<Box<XmlElement>>,
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
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
    /// element.set_tag("Div");
    /// ```
    pub fn set_tag(&mut self, tag: &str) {
        self.tag = String::from(tag);
    }

    /// Sets the text content of the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
    /// element.set_text("Hello World");
    /// ```
    pub fn set_text(&mut self, text: &str) {
        self.text = String::from(text);
    }

    /// Sets the attributes of the XmlElement.
    /// Examples:
    /// ```rust
    /// use std::collections::HashMap;
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
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
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
    /// let child = iced_xml::xml_struct::xml_element::XmlElement::void();
    /// element.set_children(vec![Box::new(child)]);
    /// ```
    pub fn set_children(&mut self, children: Vec<Box<XmlElement>>) {
        self.children = children;
    }

    /// Adds a rule the theme of the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
    /// element.apply_css("bg", "red");
    /// ```
    pub fn apply_css(&mut self, key: &str, value: &str) {
        gen_styles_log(
            &String::from(key),
            &String::from(value),
            &mut self.theme,
            &Fonts::default(),
        );
    }

    /// Appends a child to the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
    /// let child = iced_xml::xml_struct::xml_element::XmlElement::void();
    /// element.append_child(child);
    /// ```
    pub fn append_child(&mut self, child: XmlElement) {
        self.children.push(Box::new(child));
    }

    /// Removes a child from the XmlElement by index.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
    /// let child = iced_xml::xml_struct::xml_element::XmlElement::void();
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
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
    /// let child = iced_xml::xml_struct::xml_element::XmlElement::void();
    /// element.append_child(child);
    /// assert_eq!(element.children_count(), 1);
    /// ```
    pub fn children_count(&self) -> usize {
        self.children.len()
    }

    /// Sets the id of the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
    /// element.set_id("my-id");
    /// ```
    pub fn set_id(&mut self, id: &str) {
        self.id = Some(String::from(id));
    }

    /// Clears the id of the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
    /// element.set_id("my-id");
    /// element.clear_id();
    /// ```
    pub fn clear_id(&mut self) {
        self.id = None;
    }

    /// Adds a class to the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
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
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
    /// element.add_class("my-class");
    /// element.remove_class("my-class");
    /// ```
    pub fn remove_class(&mut self, class: &str) {
        self.classes.retain(|c| c != class);
    }

    /// Adds a data attribute to the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
    /// element.add_data("key", "value");
    /// ```
    pub fn add_data(&mut self, key: &str, value: &str) {
        self.datas.insert(String::from(key), String::from(value));
    }

    /// Removes a data attribute from the XmlElement.
    /// Examples:
    /// ```rust
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
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
    /// let mut element = iced_xml::xml_struct::xml_element::XmlElement::void();
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
