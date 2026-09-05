//! Main element engine for Iced-XML. This module is responsible for parsing XML, managing the DOM, and handling events.
//! TODO: Doc
use std::io::Cursor;

use crate::app_manager::ComponentFunctions;
use crate::dom::events::{DomInternalMessageType, DomMessage};
use crate::dom::query_builder::{EventResponse, EventType, QueryResponse};
use crate::rs_utils::get_unique_id;
use crate::xml_struct::element_renderer::ElementRenderer;
use crate::xml_struct::parser::{XmlElement, XmlParser};
use crate::xml_struct::theming::Fonts;
use iced::window;
use quick_xml::Reader;

///  Message is the main event-system message for Iced.
///
///  This enum is used to pass events from the UI to the engine.
///
///  It has:
#[derive(Debug, Clone, Hash)]
pub enum Message {
    ///  - DomEvent: these are the events that are meant to be handled by the engine. i32 => event UID, EventResponse => event data
    DomEvent(Option<i32>, EventResponse),
    ///  - Void: this is a event that does nothing.
    Void,
    ///  - OpenWindow: this opens a new window with the given creation params ID.
    OpenWindow(i32), // creation params ID
    ///  - WindowOpened: this is a event that is sent when a window is opened. It's an internal event that is meant to be handled by the window system. It has the window ID as parameter.
    ///  The WindowOpened handles after-window-creation setup for the Window System
    WindowOpened(window::Id, i32), // WindowID, creation params ID
    ///  - WindowClosed: this is a event that is sent when a window is closed.
    WindowClosed(window::Id),
}

///  The dynamic events are the events that are created using "Subscribe" in Iced.
///
///  These are events that aren't triggered by the user, but by a time-based event.
#[derive(Debug, Clone, Hash)]
pub enum DynamicEvent {
    ///  - SetTimeout: the query's settimeout event
    SetTimeout(i32), // time in milliseconds
    ///  - SetInterval: the query's setinterval event
    SetInterval(i32), // time in milliseconds
}

///  The EngineSettings struct is used to configure the XmlEngine, before it's created.
#[derive(Debug, Clone)]
pub struct EngineSettings {
    ///  The fonts that will be available for the styling system
    pub fonts: Fonts,
    ///  The functions that will be available for the component system (since their type is unknown for the library)
    pub functions: ComponentFunctions,
}

///  The XmlEngine is the rendering / event-handling engine for Iced-XML. It is responsible for parsing XML, managing the DOM, and handling events.
pub struct XmlEngine {
    ///  The UID of the element renderer root element. This is the root of the DOM tree. Used as "entrypoint"
    root_uid: i32,
    ///  The element renderer is the main component that handles the rendering of the DOM tree. It is responsible for managing the elements, their properties, and their events.
    pub element_renderer: ElementRenderer,
    ///  The fired events are the events that have been fired by the UI or dynamic events, they are stored and then their IDs are used to identify the callbacks.
    pub fired_events: Vec<(Option<i32>, EventResponse)>,
    ///  The dynamic events are the events that are created using "Subscribe" in Iced. This is used to store the dynamic events that needs to be "subscribed"
    pub dyn_events: Vec<(i32, DynamicEvent)>, // (Callback UID, DynamicEvent)
}

///  This stores all the reason why the XmlEngine failed to be created.
#[derive(Debug)]
pub enum XmlEngineError {
    ///  The XML failed to parse, this is usually due to invalid XML syntax.
    ///
    ///  The quick_xml::Error is the error that was returned by the quick_xml parser.
    XmlParseError(quick_xml::Error),
    ///
    ///  The root element is not a <Window> element. This is usually due to invalid XML structure.
    ///
    ///  The XmlElement is the root element that was parsed from the XML.
    InvalidRootElement(XmlElement),
}

impl XmlEngine {
    ///  This created the XmlEngine from the given XML content and settings. It will panic if the XML is invalid or if the root element is not a <Window> element.
    ///
    ///  xml: The XML content to parse. This should be a valid XML document with a <Window> root element.
    ///
    ///  settings: The settings to use for the engine. You can use the DEFAULT_ENGINE_SETTINGS constant given from window_manager! macro, and update it if needed.
    pub fn new(xml: String, settings: EngineSettings) -> Self {
        let self_op = Self::try_new(xml, settings);
        if self_op.is_err() {
            match self_op.err().unwrap() {
                XmlEngineError::XmlParseError(e) => {
                    panic!("Failed to parse XML: {}", e);
                }
                XmlEngineError::InvalidRootElement(e) => {
                    panic!("Invalid root element: {:?}", e);
                }
            }
        }
        return self_op.unwrap();
    }

    ///  This created the XmlEngine from the given XML content and settings. It will return an error if the XML is invalid or if the root element is not a <Window> element.
    ///
    ///  xml: The XML content to parse. This should be a valid XML document with a <Window> root element.
    ///
    ///  settings: The settings to use for the engine. You can use the DEFAULT_ENGINE_SETTINGS constant given from window_manager! macro, and update it if needed.
    pub fn try_new(xml: String, settings: EngineSettings) -> Result<Self, XmlEngineError> {
        // Create a quick_xml reader from the XML string
        let reader = Reader::from_reader(Cursor::new(xml.into_bytes()));
        // Create a XmlParser from the quick_xml reader
        let window_parser = XmlParser::new(&mut reader.clone(), &Fonts::new());
        // Checks
        if window_parser.is_err() {
            return Err(XmlEngineError::XmlParseError(window_parser.err().unwrap()));
        }
        let root = window_parser.unwrap().root;
        if root.tag != "Window" {
            return Err(XmlEngineError::InvalidRootElement(root));
        }

        // Create the element renderer and initialize it with the root element
        let mut element_renderer = ElementRenderer::new(settings.fonts, settings.functions);
        let uid = element_renderer.init_element_from_xml(&root, get_unique_id());

        return Ok(Self {
            dyn_events: Vec::new(),
            element_renderer: element_renderer,
            root_uid: uid,
            fired_events: Vec::new(),
        });
    }

    pub fn update(&mut self, message: Message) -> Vec<(Option<i32>, EventResponse)> {
        // Clear the fired events vector, since we are going to fill it with the new events that have been fired.
        self.fired_events.clear();
        match message {
            // This is the only event that is meant to be handled by the engine. It is used to handle events that are fired by the UI or dynamic events.
            Message::DomEvent(event_uid, mut event_data) => {
                // If the event has a next_timeout, it means that it is a dynamic event, so we need to set the event type to Dynamic.
                let mut event_type = event_data.event_type;
                if event_data.next_timeout.is_some() {
                    event_type = EventType::Dynamic;
                }

                // Handle the event based on its type.
                match event_type {
                    // Dynamic events are events that are triggered by the subscribtion system.
                    EventType::Dynamic => {
                        event_data.timer_id = event_uid;
                        self.fired_events.push((event_uid, event_data.clone()));
                    }
                    // User events are events that are triggered by the user, such as clicks, key presses, etc.
                    EventType::User => {
                        // If the event_uid is Some, it means that the event is from a registered event listener, so we need to find the event listener and call its handlers.
                        if event_uid.is_none() {
                            for event_listener in self.element_renderer.event_listeners.iter_mut() {
                                if event_listener.event_uid == event_uid.unwrap() {
                                    for handler in event_listener.handlers.iter() {
                                        self.fired_events
                                            .push((Some(handler.clone()), event_data.clone()));
                                    }
                                }
                            }
                        }
                        // If it's none, it means that the event is a direct event from the program, so we need to pass it to the element renderer to handle it.
                        if event_data.target_uid.is_some() {
                            self.element_renderer.pass_event_to_element(
                                event_data.target_uid.unwrap(),
                                event_data.event_name.clone(),
                                event_data.clone(),
                            );
                        }
                    }
                }
            }
            _ => {} // Other system events
        };

        return self.fired_events.clone();
    }

    ///  This function returns the Iced Element that represents the root of the DOM tree. This is used to render the UI in Iced.
    pub fn view(&self) -> iced::Element<'_, Message> {
        return self
            .element_renderer
            .render_element(self.root_uid, None)
            .into();
    }

    ///  Handles the events that are sent from the program to the engine.
    ///
    ///  Parameters:
    ///
    ///  - query: The DomMessage that contains the event to handle.
    ///
    ///  Returns:
    ///
    ///  - QueryResponse: The response of the event handling. It contains information about the success, and other data that might be needed by the program.
    pub fn client_events(&mut self, query: &DomMessage) -> QueryResponse {
        match &query.message {
            // If the event is a generic event, non-related to a specific element, we handle it in the generic event handler.
            DomInternalMessageType::SubscribeDynamicEvent(_)
            | DomInternalMessageType::ImportCss(_, _) => {
                return self.handle_generic_event(query);
            }
            // Else, we handle the event in the element-specific event handler.
            _ => {
                return self.handle_element_specific_event(query);
            }
        }
    }

    ///  Handles the generic events (not related to a specific element) that are sent from the program to the engine.
    ///
    ///  Parameters:
    ///
    ///  - event: The DomMessage that contains the event to handle.
    ///
    ///  Returns:
    ///
    ///  - QueryResponse: The response of the event handling. It contains information about the success, and other data that might be needed by the program.
    fn handle_generic_event(&mut self, event: &DomMessage) -> QueryResponse {
        return match &event.message {
            // If the event is a SubscribeDynamicEvent (EventType::Dynamic), we add it to the dyn_events vector, so that it can be handled later when the dynamic event is triggered.
            DomInternalMessageType::SubscribeDynamicEvent(dynamic_event) => {
                if event.uid.is_some() {
                    match dynamic_event {
                        DynamicEvent::SetInterval(time) => {
                            self.dyn_events
                                .push((event.uid.unwrap(), DynamicEvent::SetInterval(*time)));
                        }
                        DynamicEvent::SetTimeout(time) => self
                            .dyn_events
                            .push((event.uid.unwrap(), DynamicEvent::SetTimeout(*time))),
                    };
                    return QueryResponse::success();
                } else {
                    return QueryResponse::fail("Dynamic event must have a UID to be registered.");
                }
            }
            // If it's an ImportCss event, we load the CSS into the element renderer, and return the success status and any error message that might have occurred during the loading process.
            DomInternalMessageType::ImportCss(css, for_hot_reload) => {
                let (success, message) =
                    self.element_renderer.load_css(&css, for_hot_reload.clone());

                let mut query_response = QueryResponse::new(success);
                query_response.error_message = Some(message);
                return query_response;
            }
            _ => QueryResponse::fail("Message supposed to be a generic event, but it is not."),
        };
    }

    ///  Handles the element-specific events (related to a specific element) that are sent from the program to the engine.
    ///
    ///  Parameters:
    ///
    ///  - query: The DomMessage that contains the event to handle.
    ///
    ///  Returns:
    ///
    ///  - QueryResponse: The response of the event handling. It contains information about the success and other data that might be needed by the program.
    fn handle_element_specific_event(&mut self, query: &DomMessage) -> QueryResponse {
        let mut response = QueryResponse::success();
        let elements = self.element_renderer.element_query(&query.selector);
        for element in elements {
            let status = match query.message {
                DomInternalMessageType::RegisterEventListener(ref event_name) => {
                    if query.uid.is_none() {
                        self.element_renderer.register_event(
                            event_name.clone(),
                            element,
                            query.uid.unwrap(),
                        );
                        return QueryResponse::success();
                    } else {
                        return QueryResponse::fail(
                            "Event listener must have a UID to be registered.",
                        );
                    }
                }
                // This removes an element from the DOM tree. It will also remove all of its children, and any event listeners that are registered on it or its children.
                DomInternalMessageType::Remove => {
                    let source = self.element_renderer.remove_cascade(element, true);
                    let mut qr = QueryResponse::success();
                    qr.data_element = source;
                    qr
                }
                // This replaces an element in the DOM tree with another element. It will also remove all of its children, and any event listeners that are registered on it or its children.
                DomInternalMessageType::Replace(ref replacing_element) => {
                    let selector = self
                        .element_renderer
                        .replace_element(element, replacing_element.value().clone());
                    let mut qr = QueryResponse::success();
                    qr.data_selector = Some(selector);
                    qr
                }
                // This returns the "source" of the element, which is the XmlElement that represents the element in the DOM tree.
                DomInternalMessageType::GetElement => {
                    let mut qr = QueryResponse::success();
                    qr.data_element = self.element_renderer.get_source(element);
                    qr
                }
                // This returns the data associated with the element, which is a key-value store that can be used to store arbitrary data on the element.
                DomInternalMessageType::GetData(ref key) => {
                    let data = self.element_renderer.get_data(element, &key.clone());
                    if data.is_some() {
                        let mut qr = QueryResponse::success();
                        qr.data_str = data;
                        qr
                    } else {
                        QueryResponse::fail("Data not found for the given key.")
                    }
                }
                _ => QueryResponse::fail(
                    "Message supposed to be an element-specific event, but it is not.",
                ),
            };
            response.concat(status);
        }
        return response;
    }
}
