use log::error;
use std::{any::Any, time::Duration};

use iced::{Subscription, time};

use crate::{
    app_manager::{state_wrapper::App, utils::WindowId},
    dom::{
        events::DomQueryBuilder,
        query::{DomQuery, DomQueryType},
        types::{DomInternalMessageType, DomMessage, EventResponse, EventType, QueryResponse},
    },
    rs_utils::get_unique_id,
    xml_engine::{DynamicEvent, Message, XmlEngine},
};

/// Represents a query with the Query Builder.
pub struct Query<WindowState, AppState> {
    pub query: DomMessage,
    pub callback: Option<fn(&mut WindowState, QueryResponse)>,
    pub listener_callbacks: Option<Vec<fn(&mut WindowState, EventResponse, &mut App<AppState>)>>,
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
        if let Some(ev) = &query_result.event {
            let id = get_unique_id();

            let query = Query {
                query: DomMessage {
                    message: ev.message.clone(),
                    uid: Some(id),
                    selector: ev.selector.clone(),
                },
                callback: None,
                listener_callbacks: None,
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
            uid: self.last_timer_id,
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
            uid: self.last_timer_id,
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
            if last_query.listener_callbacks.is_none() {
                last_query.listener_callbacks = Some(Vec::new());
            }

            if let Some(listener_callbacks) = &mut last_query.listener_callbacks {
                listener_callbacks.push(callback);
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

        for (uid, event) in engine.dyn_events.iter() {
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
                    error!("! set_interval or set_timeout event less than 0 interval/timeout");
                }
            }
        }

        subscriptions.append(&mut engine.elements_manager.subscribe_components());

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
            if let Some(uid) = uid {
                if let Some(query) = self.queries.iter().find(|q| q.uid == uid) {
                    if let Some(listener_callbacks) = &query.listener_callbacks {
                        for callback in listener_callbacks.iter() {
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
            if query.listener_callbacks.is_none() || !query.listener_registered {
                let responses = engine.client_events(&query.query);

                if let Some(last_response) = responses.last() {
                    self.last = last_response.clone();
                }

                if let Some(callback) = query.callback {
                    for response in responses.iter() {
                        callbacks.push((callback, response.clone()));
                    }
                }

                if query.listener_callbacks.is_none() {
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
