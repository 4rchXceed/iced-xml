use std::{any::Any, time::Duration};

use iced::{Subscription, time};

use crate::{
    app_manager::{App, WindowId},
    dom::{
        events::{DomInternalMessageType, DomMessage, DomQueryBuilder, EventListenerTypes},
        query::{DomQuery, DomQueryType},
    },
    rs_utils::{
        HashableF32, HashableGridTarget, HashableHashMap, ScrollState, Vector2, get_unique_id,
    },
    xml_engine::{DynamicEvent, Message, XmlEngine},
    xml_struct::{
        elements::{textarea::TextareaEvent, window_system::TWMWindowOpenParams},
        parser::XmlElement,
    },
};

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
    pub data_element: Option<XmlElement>,
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

    /// Creates a new QueryResponse representing a successful query.
    pub fn success() -> Self {
        Self::new(true)
    }

    /// Creates a new QueryResponse representing a failed query with the given reason.
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
    pub fn with_data_element(mut self, data: XmlElement) -> Self {
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

/// Represents a query with the Query Builder.
pub struct Query<WindowState, AppState> {
    pub query: DomMessage,
    pub callback: Option<fn(&mut WindowState, QueryResponse)>,
    pub listener_callback: Option<Vec<fn(&mut WindowState, EventResponse, &mut App<AppState>)>>,
    pub listener_registered: bool,
    pub uid: i32,
}

/// Main struct for building queries to the DOM.
/// See docs for detailed usage
pub struct QueryBuilder<WindowState, AppState> {
    queries: Vec<Query<WindowState, AppState>>,
    pub last: QueryResponse,
    last_timer_id: Option<i32>,
    ignore_timer: Vec<i32>,
}

impl<Window, AppState> QueryBuilder<Window, AppState> {
    /// Creates a new QueryBuilder with no queries and a default last response.
    /// !! WITH EVERY ACTION YOU MUST CALL self.process() TO EXECUTE THE QUERY !!
    /// Self -> The WindowTemplate you're using
    pub fn new() -> Self {
        Self {
            queries: Vec::new(),
            last: QueryResponse::fail("No queries executed yet"),
            last_timer_id: None,
            ignore_timer: Vec::new(),
        }
    }

    /// Imports a CSS string into the DOM, with an option for hot reloading.
    /// Parameters:
    /// - css: The CSS string to import.
    /// - hot_reload: If true, the CSS will be reloaded automatically when it changes [Do not use, use the css-watcher feature instead].
    pub fn import_css(&mut self, css: String, hot_reload: bool) -> &mut Self {
        self.build_query(&mut DomQueryBuilder::from_dom_message(DomMessage {
            message: DomInternalMessageType::ImportCss(css, hot_reload),
            uid: Some(get_unique_id()),
            selector: DomQuery {
                query_type: DomQueryType::Unused,
                flag: None,
            },
        }));
        self
    }

    /// Builds a query to the DOM.
    /// See Dom::___.___ for more information on queries
    pub fn b(&mut self, e: &mut DomQueryBuilder) -> &mut Self {
        self.build_query(e)
    }

    /// Builds a query to the DOM.
    /// See Dom::___.___ for more information on queries
    pub fn build_query(&mut self, query_result: &mut DomQueryBuilder) -> &mut Self {
        if query_result.event.is_some() {
            let ev = query_result.event.as_ref().unwrap().clone();
            let id = get_unique_id();
            let query = Query {
                query: DomMessage {
                    message: ev.message,
                    uid: Some(id),
                    selector: ev.selector,
                },
                callback: None,
                listener_callback: None,
                listener_registered: false,
                uid: id,
            };
            self.queries.push(query);
        }
        self
    }

    /// Opens a new window with the given parameters.
    /// Parameters:
    /// - window_params: The parameters for the new window.
    /// - app_state: The application state, used to manage the window.
    pub fn open_window(
        &mut self,
        window_params: Box<dyn Any>,
        app_state: &mut App<AppState>,
    ) -> &mut Self {
        app_state.open_window(window_params);
        self.set_timeout(0); // TODO: This is a hack to make sure the window is opened right away
        self
    }

    /// Closes a window with the given ID.
    /// Parameters:
    /// - window_id: The ID of the window to close.
    /// - app_state: The application state, used to manage the window.
    pub fn close_window(
        &mut self,
        window_id: WindowId,
        app_state: &mut App<AppState>,
    ) -> &mut Self {
        app_state.close_window(window_id);
        self.set_timeout(0); // TODO: Same as above
        self
    }

    /// Works the same as setTimeout in JS. Call .with_callback() to set a callback for when the timeout is reached.
    ///
    /// Parameters:
    /// - timeout: The timeout in milliseconds.
    ///
    /// Example:
    /// ```rust,ignore
    /// self.qb.set_timeout(1000).with_callback(|window, event_response, app| {
    ///     // Do something after 1 second
    /// });
    /// ```
    pub fn set_timeout(&mut self, timeout: i32) -> &mut Self {
        self.last_timer_id = Some(get_unique_id());
        let dom_message = DomMessage {
            message: DomInternalMessageType::SubscribeDynamicEvent(DynamicEvent::SetTimeout(
                timeout,
            )),
            uid: Some(self.last_timer_id.unwrap()),
            selector: DomQuery {
                query_type: DomQueryType::Unused,
                flag: None,
            },
        };
        self.build_query(&mut DomQueryBuilder::from_dom_message(dom_message));
        self
    }

    /// Works the same as setInterval in JS. Call .with_callback() to set a callback for when the interval is reached.
    ///
    /// Parameters:
    /// - interval: The interval in milliseconds.
    ///
    /// Example:
    /// ```rust,ignore
    /// self.qb.set_interval(1000).with_callback(|window, event_response, app| {
    ///     // Do something every 1 second
    /// });
    /// ```
    pub fn set_interval(&mut self, interval: i32) -> &mut Self {
        self.last_timer_id = Some(get_unique_id());
        let dom_message = DomMessage {
            message: DomInternalMessageType::SubscribeDynamicEvent(DynamicEvent::SetInterval(
                interval,
            )),
            uid: Some(self.last_timer_id.unwrap()),
            selector: DomQuery {
                query_type: DomQueryType::Unused,
                flag: None,
            },
        };
        self.build_query(&mut DomQueryBuilder::from_dom_message(dom_message));
        self
    }

    /// Gets the last timer ID set by set_timeout or set_interval.
    /// Used for clear_timer() to clear the timer.
    /// Same as JS let timer_id = setTimeout(...); clearTimeout(timer_id);
    pub fn get_timer_id(&mut self, id: &mut Option<i32>) -> &mut Self {
        *id = self.last_timer_id;
        self
    }

    /// Clears a timer set by set_timeout or set_interval.
    /// Aka cancels it. Same as JS clearTimeout(timer_id); or clearInterval(timer_id);
    ///
    /// Example:
    /// ```rust,ignore
    /// let mut timer_id: Option<i32> = None;
    /// self.qb.set_timeout(1000).get_timer_id(&mut timer_id);
    /// self.process();
    /// self.qb.clear_timer(timer_id.unwrap());
    /// self.process();
    /// ```
    pub fn clear_timer(&mut self, timer_id: i32) -> &mut Self {
        self.ignore_timer.push(timer_id);
        self
    }

    /// Adds a callback to the last query, note that if the query doesn't support callbacks, this will do nothing.
    ///
    /// Parameters:
    /// - callback: The callback function to be called when the query is executed. (!! MUST BE STATIC !!)
    ///
    /// Example:
    /// ```rust,ignore
    /// self.qb.set_timeout(1000).with_callback(|window, event_response, app| {
    ///     // Do something after 1 second
    /// });
    /// ```
    pub fn with_callback(
        &mut self,
        callback: fn(&mut Window, EventResponse, &mut App<AppState>),
    ) -> &mut Self {
        if let Some(last_query) = self.queries.last_mut() {
            if last_query.listener_callback.is_none() {
                last_query.listener_callback = Some(Vec::new());
            }
            if last_query.listener_callback.is_some() {
                last_query
                    .listener_callback
                    .as_mut()
                    .unwrap()
                    .push(callback);
            }
        }
        self
    }

    /// Runs a callback when the action is processed (.process() is called).
    ///
    /// Please note: if you need only the last query's data, you can: `let last_query = self.process();` (or `self.qb.last`).
    ///
    /// Parameters:
    /// - callback: the callback function, with two arguments: The current class, the response data (!! MUST BE STATIC !!)
    ///
    /// Example:
    /// ```rust,ignore
    /// self.qb.b(Dom::get_element_by_id("test").get_property("value")).then(|window, query_response| {
    ///     println!("Property value: {:?}", query_response.data_str);
    /// });
    /// self.process();
    /// ```
    pub fn then(&mut self, callback: fn(&mut Window, QueryResponse)) -> &mut Self {
        if let Some(last_query) = self.queries.last_mut() {
            last_query.callback = Some(callback);
        }
        self
    }

    /// Internal function used by the pre-defined functions in the WindowTemplate.
    /// See iced subscription system for more information.
    pub fn subscribe(&self, engine: &XmlEngine) -> Vec<Subscription<Message>> {
        let mut subscriptions = Vec::new();
        for dynamic_event in engine.dyn_events.iter() {
            let (uid, event) = dynamic_event;
            if !self.ignore_timer.contains(uid) {
                let every: i32;
                let mut ev_data = EventResponse::default();
                match event {
                    DynamicEvent::SetInterval(interval) => {
                        every = *interval;
                        ev_data.next_timeout = Some(*interval as u64);
                    }
                    DynamicEvent::SetTimeout(timeout) => {
                        every = *timeout;
                        ev_data.event_type = EventType::Dynamic;
                    }
                };
                if every >= 0 {
                    let ev_uid = uid.clone();
                    subscriptions.push(
                        time::every(Duration::from_millis(every.clone() as u64))
                            .with(Message::DomEvent(Some(ev_uid), ev_data.clone()))
                            .map(|a| a.0),
                    );
                } else {
                    println!("! set_interval or set_timeout event less than 0 interval/timeout");
                }
            }
        }
        subscriptions.append(&mut engine.element_renderer.subscribe_components());
        return subscriptions;
    }

    /// Internal function used by the pre-defined functions in the WindowTemplate.
    ///
    /// This one is returning all the callbacks that need to be executed, with the EventResponse data.
    /// These events can be user-generated (click, ...) or dynamic (set_timeout, set_interval, ...).
    pub fn fetch(
        &mut self,
        returned_callbacks: Vec<(Option<i32>, EventResponse)>,
    ) -> Vec<(
        fn(&mut Window, EventResponse, &mut App<AppState>),
        EventResponse,
    )> {
        let mut callbacks = Vec::new();
        for (uid, event_response) in returned_callbacks {
            if uid.is_some() {
                if let Some(query) = self.queries.iter().find(|q| q.uid == uid.unwrap()) {
                    if query.listener_callback.is_some() {
                        for callback in query.listener_callback.as_ref().unwrap().iter() {
                            callbacks.push((*callback, event_response.clone()));
                        }
                    }
                }
            }
        }
        return callbacks;
    }

    /// Internal function used by the pre-defined functions in the WindowTemplate.
    ///
    /// This one is returning all the callbacks that need to be executed, with the QueryResponse data.
    /// These events are QueryBuilder-generated (get_property, set_property, ...).
    pub fn execute(
        &mut self,
        engine: &mut XmlEngine,
    ) -> Vec<(fn(&mut Window, QueryResponse), QueryResponse)> {
        let mut callbacks = Vec::new();
        let mut queries_to_remove: Vec<usize> = Vec::new();
        let mut i: usize = 0;
        for query in self.queries.iter_mut() {
            if query.listener_callback.is_none() || !query.listener_registered {
                let responses = engine.client_events(&query.query);
                if responses.len() > 0 {
                    self.last = responses.last().unwrap().clone();
                }
                if query.callback.is_some() {
                    for response in responses.iter() {
                        callbacks.push((query.callback.unwrap(), response.clone()));
                    }
                }
                if query.listener_callback.is_none() {
                    queries_to_remove.push(i);
                } else {
                    query.listener_registered = true;
                }
            }
            i += 1;
        }
        for index in queries_to_remove.iter().rev() {
            self.queries.remove(*index);
        }
        return callbacks;
    }
}
