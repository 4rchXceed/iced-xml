use std::time::Duration;

use iced::{Subscription, time};

use crate::{
    dom::events::{DomInternalMessageType, DomMessage, DomQuery, DomQueryType},
    xml_engine::{DynamicEvent, Message, XmlEngine},
};

#[derive(Debug, Clone, Hash)]
pub struct EventResponse {
    // HERE: All properties in Option<> for every event response, so that we can return None if the event is not applicable to the element
    pub next_timeout: Option<u64>,
    pub is_timeout: bool,
    pub target: Option<DomQuery>,
}

impl EventResponse {
    pub fn new(uid: i32) -> Self {
        Self {
            next_timeout: None,
            is_timeout: false,
            target: Some(DomQuery {
                query_type: DomQueryType::ByUid(uid),
                flag: None,
            }),
        }
    }
}

impl Default for EventResponse {
    fn default() -> Self {
        Self {
            next_timeout: None,
            is_timeout: false,
            target: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueryResponse {
    pub success: bool,
    pub element_uid: Option<i32>,
    pub error_message: Option<String>,
    pub data_str: Option<String>,
    pub data_bool: Option<bool>,
}

impl QueryResponse {
    pub fn new(success: bool) -> Self {
        Self {
            success,
            element_uid: None,
            error_message: None,
            data_str: None,
            data_bool: None,
        }
    }
}

pub struct Query<T> {
    pub query: DomMessage,
    pub callback: Option<fn(&mut T, QueryResponse)>,
    pub listener_callback: Option<fn(&mut T, EventResponse)>,
    pub listener_registered: bool,
    pub uid: i32,
}

pub struct QueryBuilder<T> {
    queries: Vec<Query<T>>,
    current_uid: i32,
    pub last: QueryResponse,
}

impl<T> QueryBuilder<T> {
    pub fn new() -> Self {
        Self {
            queries: Vec::new(),
            current_uid: 0,
            last: QueryResponse::new(false),
        }
    }

    pub fn import_css(&mut self, css: String, hot_reload: bool) -> &mut Self {
        self.build_query(DomMessage {
            message: DomInternalMessageType::ImportCss(css, hot_reload),
            uid: self.current_uid,
            selector: DomQuery {
                query_type: DomQueryType::Unused,
                flag: None,
            },
        });
        self
    }

    pub fn b(&mut self, e: DomMessage) -> &mut Self {
        self.build_query(e)
    }

    pub fn build_query(&mut self, event: DomMessage) -> &mut Self {
        let query = Query {
            query: DomMessage {
                message: event.message,
                uid: self.current_uid,
                selector: event.selector,
            },
            callback: None,
            listener_callback: None,
            listener_registered: false,
            uid: self.current_uid,
        };
        self.current_uid += 1;
        self.queries.push(query);
        self
    }

    pub fn set_timeout(&mut self, timeout: i32) -> &mut Self {
        self.build_query(DomMessage {
            message: DomInternalMessageType::SubscribeDynamicEvent(DynamicEvent::SetTimeout(
                timeout,
            )),
            uid: self.current_uid,
            selector: DomQuery {
                query_type: DomQueryType::Unused,
                flag: None,
            },
        });
        self
    }

    pub fn set_interval(&mut self, interval: i32) -> &mut Self {
        self.build_query(DomMessage {
            message: DomInternalMessageType::SubscribeDynamicEvent(DynamicEvent::SetInterval(
                interval,
            )),
            uid: self.current_uid,
            selector: DomQuery {
                query_type: DomQueryType::Unused,
                flag: None,
            },
        });
        self
    }

    pub fn with_callback(&mut self, callback: fn(&mut T, EventResponse)) -> &mut Self {
        if let Some(last_query) = self.queries.last_mut() {
            last_query.listener_callback = Some(callback);
        }
        self
    }

    pub fn then(&mut self, callback: fn(&mut T, QueryResponse)) -> &mut Self {
        if let Some(last_query) = self.queries.last_mut() {
            last_query.callback = Some(callback);
        }
        self
    }

    pub fn subscribe(&self, engine: &XmlEngine) -> Subscription<Message> {
        let mut subscriptions = Vec::new();
        for dynamic_event in engine.dyn_events.iter() {
            let (uid, event) = dynamic_event;
            let every: i32;
            let mut ev_data = EventResponse::default();
            match event {
                DynamicEvent::SetInterval(interval) => {
                    every = *interval;
                    ev_data.next_timeout = Some(*interval as u64);
                }
                DynamicEvent::SetTimeout(timeout) => {
                    every = *timeout;
                    ev_data.is_timeout = true;
                }
            };
            if every >= 0 {
                let ev_uid = uid.clone();
                subscriptions.push(
                    time::every(Duration::from_millis(every.clone() as u64))
                        .with(Message::DomEvent(ev_uid, ev_data.clone()))
                        .map(|a| a.0),
                );
            } else {
                println!("! set_interval or set_timeout event less than 0 interval/timeout");
            }
        }
        return Subscription::batch(subscriptions);
    }

    pub fn fetch(
        &mut self,
        returned_callbacks: Vec<(i32, EventResponse)>,
    ) -> Vec<(fn(&mut T, EventResponse), EventResponse)> {
        let mut callbacks = Vec::new();
        for (uid, event_response) in returned_callbacks {
            if let Some(query) = self.queries.iter().find(|q| q.uid == uid) {
                if let Some(callback) = query.listener_callback {
                    callbacks.push((callback, event_response));
                }
            }
        }
        return callbacks;
    }

    pub fn execute(
        &mut self,
        engine: &mut XmlEngine,
    ) -> Vec<(fn(&mut T, QueryResponse), QueryResponse)> {
        let mut callbacks = Vec::new();
        let mut queries_to_remove: Vec<usize> = Vec::new();
        let mut i: usize = 0;
        for query in self.queries.iter_mut() {
            if query.listener_callback.is_none() || !query.listener_registered {
                let response = engine.client_events(&query.query);
                self.last = response.clone();
                if let Some(callback) = query.callback {
                    callbacks.push((callback, response));
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
