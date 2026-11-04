use crate::{
    css::types::SelectorType,
    dom::{
        query::DomQuery,
        types::CustomElementEvent,
        types::{DomInternalMessageType, DomMessage, EventListenerTypes},
    },
    elements_manager::{elements_renderer::ElementsRenderer, utils::StyleChangeEvent},
    rs_utils::HashableXmlElement,
    xml_struct::xml_element::XmlElement,
};

/// A result of a Dom::XYZ query, which can be used to select elements (View Dom struct & src/dom/api.rs)
#[derive(Debug, Clone)]
pub struct DomQueryBuilder {
    query_event: DomQuery,
    pub event: Option<DomMessage>,
}

impl DomQueryBuilder {
    /// Creates a new DomQueryResult with the given query type and element
    ///
    /// Parameters:
    /// - query_type: The SelectorType for the query
    /// - element: The "value" of the query
    ///
    /// Returns:
    /// - A new Self
    pub fn new(query_type: SelectorType, element: String) -> Self {
        Self {
            query_event: DomQuery::new(query_type, element, None),
            event: None,
        }
    }

    /// Returns the DomQuery associated with this result
    pub(crate) fn get_query(&self) -> &DomQuery {
        return &self.query_event;
    }

    /// Adds a flag to the query,if  it's a ::for() flag, it will be added to the event instead of the query
    pub fn with_flag(&mut self, flag: String) -> &mut Self {
        let style_flag = ElementsRenderer::extract_selector_style_flag(&flag);

        if style_flag.is_some() {
            if let Some(event) = &mut self.event {
                self.event = Some(DomMessage {
                    message: event.message.clone(),
                    uid: event.uid,
                    selector: event.selector.clone(),
                });
            }
        } else {
            self.query_event.flag = Some(flag);
        }

        return self;
    }

    /// Creates a new DomQueryResult from a DomQuery
    pub fn from(dom_query: DomQuery) -> Self {
        return Self {
            query_event: dom_query,
            event: None,
        };
    }

    /// Creates a new DomQueryResult from a DomMessage
    pub fn from_dom_message(dom_message: DomMessage) -> Self {
        return Self {
            query_event: dom_message.selector.clone(),
            event: Some(dom_message),
        };
    }

    /// Sets a property on the element selected by the query
    ///
    /// Parameters:
    /// - key: The property name
    /// - value: The property value
    ///
    /// Example:
    /// ```rust,ignore
    /// self.qb.b(Dom::get_element_by_id("test").set_property("value", "new_value"));
    /// self.process();
    /// ```
    pub fn set_property(&mut self, key: &str, value: &str) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::PropertyChange(key.to_string(), value.to_string()),
            uid: None,
            selector: self.query_event.clone(),
        };

        self.event = Some(event);

        return self;
    }

    /// Gets a property from the element selected by the query
    ///
    /// Parameters:
    /// - key: The property name
    /// Example:
    /// ```rust,ignore
    /// self.qb.b(Dom::get_element_by_id("test").get_property("value")).then(|query_response| {
    ///     println!("Property value: {:?}", query_response.data_str);
    /// });
    /// self.process();
    /// // Some("Property value: Some(\"new_value\")")
    /// ```
    pub fn get_property(&mut self, key: &str) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::GetProperty(key.to_string()),
            uid: None,
            selector: self.query_event.clone(),
        };

        self.event = Some(event);

        return self;
    }

    /// Sets a style on the element selected by the query
    ///
    /// Parameters:
    /// - key: The style property name
    /// - value: The style property value
    /// Example:
    /// ```rust,ignore
    /// self.qb.b(Dom::get_element_by_id("test").set_style("bg", "red"));
    /// self.process();
    /// ```
    pub fn set_style(&mut self, key: &str, value: &str) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::StyleChange(StyleChangeEvent {
                key: key.to_string(),
                value: value.to_string(),
                custom_flag: None,
            }),
            uid: None,
            selector: self.query_event.clone(),
        };

        self.event = Some(event);

        return self;
    }

    /// Registers an event listener on the element selected by the query
    ///
    /// Parameters:
    /// - name: The event name
    ///
    /// Example:
    /// ```rust,ignore
    /// self.qb.b(Dom::get_element_by_id("test").add_event_listener("click")).with_callback(|query_response| ...);
    /// self.process();
    /// ```
    pub fn add_event_listener(&mut self, name: EventListenerTypes) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::RegisterEventListener(name),
            uid: None,
            selector: self.query_event.clone(),
        };

        self.event = Some(event);

        return self;
    }

    /// Gets a value from the element dataset (data-* attributes)
    ///
    /// Parameters:
    /// - key: The dataset key
    /// Example:
    /// ```rust,ignore
    /// self.qb.b(Dom::get_element_by_id("test").get_data("value")).then(|query_response| {
    ///     println!("Dataset value: {:?}", query_response.data_str);
    /// });
    /// self.process();
    /// // <Test id="test" data-value="new_value" />
    /// ```
    pub fn get_data(&mut self, key: &str) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::GetData(key.to_string()),
            uid: None,
            selector: self.query_event.clone(),
        };

        self.event = Some(event);

        return self;
    }

    /// Fires a custom event on the element selected by the query
    ///
    /// Parameters:
    /// - event: The custom event to fire
    /// Example:
    /// ```rust,ignore
    /// self.qb.b(Dom::get_element_by_id("test").fire_event(CustomElementEvent::new("custom_event", Some("event_data"))));
    /// self.process();
    /// ```
    pub fn fire_event(&mut self, event: CustomElementEvent) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::FireEvent(event),
            uid: None,
            selector: self.query_event.clone(),
        };

        self.event = Some(event);

        return self;
    }

    /// Removes the element selected by the query from the DOM
    ///
    /// Example:
    /// ```rust,ignore
    /// self.qb.b(Dom::get_element_by_id("test").remove());
    /// self.process();
    /// ```
    pub fn remove(&mut self) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::Remove,
            uid: None,
            selector: self.query_event.clone(),
        };

        self.event = Some(event);

        return self;
    }

    /// Replaces the element selected by the query with a new element
    ///
    /// Parameters:
    /// - new_element: The new XmlElement to replace the old one with
    /// Example:
    /// ```rust,ignore
    /// let new_element = xml("<Test></Test>")
    /// self.qb.b(Dom::get_element_by_id("test").replace(new_element));
    /// self.process();
    /// ```
    pub fn replace(&mut self, new_element: Box<XmlElement>) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::Replace(HashableXmlElement::new(new_element)),
            uid: None,
            selector: self.query_event.clone(),
        };

        self.event = Some(event);

        return self;
    }

    /// Gets the element's source (XmlElement) from the DOM
    pub fn get_element(&mut self) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::GetElement,
            uid: None,
            selector: self.query_event.clone(),
        };

        self.event = Some(event);

        return self;
    }
}
