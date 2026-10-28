use crate::{
    dom::{
        api::Dom,
        events::DomQueryBuilder,
        query::{DomQuery, DomQueryType},
    },
    elements_manager::utils::StyleChangeEvent,
    rs_utils::{
        HashableF32, HashableGridTarget, HashableHashMap, HashableXmlElement, ScrollState, Vector2,
    },
    xml_engine::DynamicEvent,
    xml_struct::{
        elements::{textarea::TextareaEvent, window_system::TWMWindowOpenParams},
        xml_element::XmlElement,
    },
};

/// Message to pass to the Engine / Element Renderer from the "Client" side (like the UI or the user code)
#[derive(Debug, Clone, Hash)]
pub struct DomMessage {
    /// The type of message to send to the engine
    pub message: DomInternalMessageType,
    /// The unique identifier of the element to send the message to (if any)
    pub uid: Option<i32>,
    /// The selector to use to find the element to send the message to
    pub selector: DomQuery,
}

/// The type of message to send to the engine
#[derive(Debug, Clone, Hash)]
pub enum DomInternalMessageType {
    StyleChange(StyleChangeEvent),  // k => v custom_style_flag [for(xyz)]
    PropertyChange(String, String), // k => v
    GetProperty(String),            // key
    RegisterEventListener(EventListenerTypes), // event_name
    ImportCss(String, bool),        // css content
    SubscribeDynamicEvent(DynamicEvent), // dynamic events (like set_timeout, set_interval, etc.)
    GetData(String),                // key
    FireEvent(CustomElementEvent),  // event name, event data
    Remove,                         // remove element
    Replace(HashableXmlElement),    // replace element with new one
    GetElement,                     // get the element's source
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum EventListenerTypes {
    Click,
    Check,
    Input,
    Paste,
    Submit,
    Release,
    Scroll,
    Select,
    OnClose,
    OnOpen,
    OnInput,
    TextareaEvent,
    Toggle,
    Hidden,
    Shown,
    Resize,
    Drag,
    Maximize,
    Restore,
    Close,
    Focus,
    None,
}

/// This represents custom events that can be fired by the user to a specific element.
/// Used with the .fire_event() method.
///
/// Example:
/// ```rust,ignore
/// self.qb.b(Dom::get_element_by_id("test").fire_event(CustomElementEvent::AddSelectOption("option1".to_string(), "Option 1".to_string())));
/// self.process();
/// ```
#[derive(Debug, Clone, Hash)]
pub enum CustomElementEvent {
    AddSelectOption(String, String),                    // (key, text)
    RemoveSelectOption(String),                         // (key)
    SetTableData(Vec<HashableHashMap<String, String>>), // (data)
    TextareaEvent(TextareaEvent),                       // (event)
    MoveCursor(Vector2),                                // (position)
    TWMMaximizeWindow(String),                          // (window)
    TWMRestoreWindow(),                                 // (no data)
    TWMCloseWindow(String),                             // (window)
    TWMFocusWindow(String),                             // (window)
    TWMDragWindow(String, HashableGridTarget),          // (window, position)
    TWMOpenWindow(TWMWindowOpenParams),                 // (window)
    ClearSelectOptions(),                               // (no data)
}

/// Two (internal) event types are possible:
/// User -> Created by the user, for example a click event.
/// Dynamic -> Created by the engine's subscription system, for example a set_timeout event.
#[derive(Debug, Clone, Hash, Copy)]
pub enum EventType {
    User,
    Dynamic,
}

/// Represents the data returned from an event
/// TODO: Future: Make this an enum with different variants for different event types, so that we can have a more type-safe way of handling events.
#[derive(Debug, Clone, Hash)]
pub struct EventResponse {
    // HERE: All properties in Option<> for every event response, so that we can return None if the event is not applicable to the element
    pub next_timeout: Option<u64>,
    pub event_type: EventType,
    pub timer_id: Option<i32>, // useful for clear_timer
    pub target_uid: Option<i32>,
    pub event_name: EventListenerTypes,
    pub target: Option<DomQuery>,
    pub data_str: Option<String>,
    pub data_bool: Option<bool>,
    pub data_int: Option<i32>,
    pub data_float: Option<HashableF32>,
    pub data_vector: Option<Vector2>,
    // Element-specific properties (with non-builtin types):
    // WindowSystem
    pub window_system_data_window: Option<iced::widget::pane_grid::Pane>,
    pub window_system_data_split: Option<iced::widget::pane_grid::Split>,
    pub window_system_data_target: Option<HashableGridTarget>,
    pub scrollable_scroll_state: Option<ScrollState>,
    pub textarea_event: Option<TextareaEvent>,
}

impl EventResponse {
    /// Creates a new EventResponse with the given target UID and event type.
    ///
    /// Parameters:
    /// - uid: The UID of the target element.
    /// - event_type: The type of the event ("click", ...).
    pub fn new(uid: i32, event_type: EventListenerTypes) -> Self {
        Self {
            target: Some(DomQuery {
                query_type: DomQueryType::ByUid(uid),
                flag: None,
            }),
            target_uid: Some(uid),
            event_name: event_type,
            ..Default::default()
        }
    }
}

impl Default for EventResponse {
    /// Default values for EventResponse
    fn default() -> Self {
        Self {
            next_timeout: None,
            event_type: EventType::User,
            target: None,
            data_str: None,
            event_name: EventListenerTypes::None,
            timer_id: None,
            target_uid: None,
            data_bool: None,
            data_int: None,
            window_system_data_window: None,
            window_system_data_split: None,
            data_float: None,
            data_vector: None,
            window_system_data_target: None,
            scrollable_scroll_state: None,
            textarea_event: None,
        }
    }
}

/// Represents the response from a query with the Query Builder.
/// Used to return data from the query.
/// For example:
/// - A success
/// - A failure with an error message
/// - A string value
#[derive(Debug, Clone)]
pub struct QueryResponse {
    pub success: bool, // If one element failed, the whole query is considered failed.
    pub detailed_success: Vec<Box<QueryResponse>>, // If one element failed, the whole query is considered failed.
    pub element_uid: Option<i32>,
    pub error_message: Option<String>,
    pub data_str: Option<String>,
    pub data_bool: Option<bool>,
    pub data_float: Option<HashableF32>,
    pub data_vector: Option<Vector2>,
    pub data_element: Option<Box<XmlElement>>,
    pub data_selector: Option<DomQuery>,
}

impl QueryResponse {
    /// Creates a new QueryResponse with the given success status.
    pub fn new(success: bool) -> Self {
        Self {
            success,
            detailed_success: Vec::new(),
            element_uid: None,
            error_message: None,
            data_str: None,
            data_bool: None,
            data_float: None,
            data_vector: None,
            data_element: None,
            data_selector: None,
        }
    }

    pub fn selector(&self) -> Option<DomQueryBuilder> {
        if let Some(element_uid) = self.element_uid {
            return Some(Dom::get_element(element_uid));
        } else {
            return None;
        }
    }

    /// Creates a new QueryResponse representing a successful query.
    pub fn success() -> Self {
        Self::new(true)
    }

    /// Creates a new QueryResponse representing a failed query with the given reason.
    /// TODO: Make the message an enum
    pub fn fail(reason: &str) -> Self {
        Self::new(false).with_error_message(reason)
    }

    /// Adds an Error Message to the Query Response
    pub fn with_error_message(mut self, message: &str) -> Self {
        self.error_message = Some(message.to_string());
        self
    }

    /// Adds a String to the Query Response
    pub fn with_data_str(mut self, data: String) -> Self {
        self.data_str = Some(data);
        self
    }

    /// Adds a Boolean to the Query Response
    pub fn with_data_bool(mut self, data: bool) -> Self {
        self.data_bool = Some(data);
        self
    }

    /// Adds an Integer to the Query Response
    pub fn with_data_float(mut self, data: f32) -> Self {
        self.data_float = Some(HashableF32::new(data));
        self
    }

    /// Adds a Vector2 to the Query Response
    pub fn with_data_vector(mut self, data: Vector2) -> Self {
        self.data_vector = Some(data);
        self
    }

    /// Adds an XmlElement to the Query Response
    pub fn with_data_element(mut self, data: Box<XmlElement>) -> Self {
        self.data_element = Some(data);
        self
    }

    /// Adds a DomQuery to the Query Response
    pub fn with_data_selector(mut self, data: DomQuery) -> Self {
        self.data_selector = Some(data);
        self
    }

    /// Returns the string data from the Query Response, or a default value if it is None.
    pub fn get_str_or(&self, default: &str) -> String {
        if let Some(data_str) = &self.data_str {
            return data_str.clone();
        }
        return default.to_string();
    }

    /// Adds another QueryResponse to the detailed_success vector, allowing for multiple responses to be stored in a single QueryResponse.
    pub fn concat(&mut self, other: QueryResponse) {
        if !other.success {
            self.success = false;
        }
        self.detailed_success.push(Box::new(other));
    }
}
