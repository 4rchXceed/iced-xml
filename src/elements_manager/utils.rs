use std::any::Any;
use std::collections::HashMap;

use crate::css::types::RuleBlock;
use crate::dom::events::EventListenerTypes;
use crate::dom::query_builder::QueryResponse;
use crate::xml_engine::Message;
use crate::xml_struct::elements::radio::RadioElement;
use crate::xml_struct::parser::XmlElement;
use crate::xml_struct::theming::XmlTheme;

/// Datas that can be passed as "parameters" to children elements
#[derive(Clone, Debug)]
pub struct RenderChildParameters {
    pub table_datas: Option<HashMap<String, String>>, // For table elements
}

impl Default for RenderChildParameters {
    fn default() -> Self {
        Self { table_datas: None }
    }
}

/// Represents a style change event (key-value pair)
/// This will be used to update the style of an element when a CSS rule is applied or changed.
#[derive(Debug, Clone, Hash)]
pub struct StyleChangeEvent {
    pub key: String,
    pub value: String,
    pub custom_flag: Option<String>,
}

/// This represents an event listener that can be attached to an element.
pub struct EventListener {
    /// See EventListenerTypes for the list of events that can be listened to
    pub event_type: EventListenerTypes,
    /// Target element UID
    pub target: i32,
    /// The list of callbacks to call when the event is triggered
    pub handlers: Vec<i32>, // List of callbacks to forward the event to
    /// Unique ID for the event
    pub event_uid: i32,
}

/// Event to fire from an element to the renderer
pub enum RendererEvent {
    // If elements wants to emit an event to the renderer, it can use this enum
    RadioSelectionChange(String, RadioElement), // (selection_id, selected_element)
}

/// This struct represents a state (for one element) at an instant T, to be able to revert style changes when hot-reloading CSS.
#[derive(Clone, Debug)]
pub struct HotReloadStateEntry {
    pub default_theme: XmlTheme,
    pub flag_themes: HashMap<String, XmlTheme>,
}

/// This struct represents the state of the renderer (style) at an instant T, to be able to revert style changes when hot-reloading CSS.
#[derive(Clone)]
pub struct HotReloadState {
    pub rules: Vec<RuleBlock>,
    pub state: HashMap<i32, HotReloadStateEntry>,
}

/// This struct represents the extra data associated with an element, such as its style, XML Source, ...
#[derive(Clone, Debug)]
pub struct ElementExtraData {
    /// The Main theme
    pub default_theme: XmlTheme,
    /// Themes for "sub-elements" (::for(XYZ))
    pub flag_themes: HashMap<String, XmlTheme>,
    /// It's XML "Source"
    pub xml_element: Box<XmlElement>,
    /// Parameters from parents
    pub child_data: Option<RenderChildParameters>,
}

/// This struct represents the response of an element to an event.
/// It contains the response to the event, the list of elements to forward the event to, and the list of events to forward to the renderer.
pub struct ElementEventResponse {
    /// Direct response to the event, can be success or fail, and carry additional data (like a string, ...)
    pub response: QueryResponse,
    /// List of elements to forward the event to
    pub forward_to: Vec<i32>, // List of elements to forward the event to
    /// List of events to forward to the renderer
    pub renderer_events: Vec<RendererEvent>, // List of events to forward to the renderer
}

impl ElementEventResponse {
    /// Creates a new ElementEventResponse with the given response
    ///
    /// Parameters:
    /// - response: the QueryResponse, useful to pass information about the event (additional data)
    pub fn new(response: QueryResponse) -> Self {
        Self {
            response: response,
            forward_to: Vec::new(),
            renderer_events: Vec::new(),
        }
    }

    /// Creates a successful event response
    pub fn success() -> Self {
        Self {
            response: QueryResponse::success(),
            forward_to: Vec::new(),
            renderer_events: Vec::new(),
        }
    }

    /// Adds elements to forward the event to
    ///
    /// Parameters:
    /// - forward_to: the list of elements to forward the event to (by UID)
    pub fn with_forward_to(mut self, forward_to: Vec<i32>) -> Self {
        self.forward_to = forward_to;
        self
    }

    /// Adds events to forward to the renderer
    ///
    /// Parameters:
    /// - renderer_events: the list of events to forward to the renderer (see RendererEvent)
    pub fn with_renderer_events(mut self, renderer_events: Vec<RendererEvent>) -> Self {
        self.renderer_events = renderer_events;
        self
    }
}

/// "Hack" to store the render and update functions for components, since they are unknown types (dyn Any)
/// Will need to figure out how to make this better
pub type RenderComponentFn = fn(&Box<dyn Any>) -> iced::Element<'_, Message>;
/// "Hack" to store the render and update functions for components, since they are unknown types (dyn Any)
/// Will need to figure out how to make this better
pub type UpdateComponentFn = fn(&Box<dyn Any>, Message) -> iced::Task<Message>;
