use std::{any::Any, collections::HashMap, time::Duration};

use iced::{Subscription, time};

use crate::{
    app_manager::{App, WindowId},
    dom::events::{DomInternalMessageType, DomMessage, DomQuery, DomQueryResult, DomQueryType},
    rs_utils::{
        HashableF32, HashableGridTarget, HashableHashMap, ScrollState, VectorWH, VectorXY,
        get_unique_id,
    },
    xml_engine::{DynamicEvent, Message, XmlEngine},
    xml_struct::elements::textarea::TextareaEvent,
};

#[derive(Debug, Clone, Hash)]
pub struct DomEvent {
    pub datas_str: HashableHashMap<String, String>,
    pub data_bool: Option<bool>,
    pub data_int: Option<i32>,
    pub data_float: Option<HashableF32>,
    pub data_tabledata: Option<Vec<HashableHashMap<String, String>>>,
    pub data_textarea_event: Option<TextareaEvent>,
    pub data_vector: Option<VectorXY>,
}

impl DomEvent {
    pub fn new() -> Self {
        Self {
            datas_str: HashableHashMap::new(HashMap::new()),
            data_bool: None,
            data_int: None,
            data_float: None,
            data_tabledata: None,
            data_textarea_event: None,
            data_vector: None,
        }
    }

    pub fn with(&mut self, key: &str, value: &str) -> &mut Self {
        self.datas_str
            .0
            .insert(key.to_string().clone(), value.to_string().clone());
        self
    }

    pub fn with_bool(&mut self, value: bool) -> &mut Self {
        self.data_bool = Some(value);
        self
    }

    pub fn with_int(&mut self, value: i32) -> &mut Self {
        self.data_int = Some(value);
        self
    }

    pub fn with_float(&mut self, value: f32) -> &mut Self {
        self.data_float = Some(HashableF32::new(value));
        self
    }

    pub fn with_tabledata(&mut self, value: Vec<HashMap<String, String>>) -> &mut Self {
        self.data_tabledata = Some(
            value
                .iter()
                .map(|v| HashableHashMap::new(v.clone()))
                .collect(),
        );
        self
    }

    pub fn with_textarea_event(&mut self, value: TextareaEvent) -> &mut Self {
        self.data_textarea_event = Some(value);
        self
    }
}

#[derive(Debug, Clone, Hash)]
pub struct EventResponse {
    // HERE: All properties in Option<> for every event response, so that we can return None if the event is not applicable to the element
    pub next_timeout: Option<u64>,
    pub is_timeout: bool,
    pub timer_id: Option<i32>, // useful for clear_timer
    pub target_uid: i32,
    pub event_type: String,
    pub target: Option<DomQuery>,
    pub data_str: Option<String>,
    pub data_bool: Option<bool>,
    pub data_int: Option<i32>,
    pub data_float: Option<HashableF32>,
    pub data_vectorwh: Option<VectorWH>,
    // Element-specific properties (with non-builtin types):
    // WindowSystem
    pub window_system_data_window: Option<iced::widget::pane_grid::Pane>,
    pub window_system_data_split: Option<iced::widget::pane_grid::Split>,
    pub window_system_data_target: Option<HashableGridTarget>,
    pub scrollable_scroll_state: Option<ScrollState>,
    pub textarea_event: Option<TextareaEvent>,
}

impl EventResponse {
    pub fn new(uid: i32, event_type: String) -> Self {
        Self {
            target: Some(DomQuery {
                query_type: DomQueryType::ByUid(uid),
                flag: None,
            }),
            target_uid: uid,
            event_type: event_type,
            ..Default::default()
        }
    }
}

impl Default for EventResponse {
    fn default() -> Self {
        Self {
            next_timeout: None,
            is_timeout: false,
            target: None,
            data_str: None,
            event_type: String::new(),
            timer_id: None,
            target_uid: -1,
            data_bool: None,
            data_int: None,
            window_system_data_window: None,
            window_system_data_split: None,
            data_float: None,
            window_system_data_target: None,
            scrollable_scroll_state: None,
            data_vectorwh: None,
            textarea_event: None,
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
    pub data_float: Option<HashableF32>,
    pub data_vector: Option<VectorXY>,
}

impl QueryResponse {
    pub fn new(success: bool) -> Self {
        Self {
            success,
            element_uid: None,
            error_message: None,
            data_str: None,
            data_bool: None,
            data_float: None,
            data_vector: None,
        }
    }

    pub fn get_str_or(&self, default: &str) -> String {
        if let Some(data_str) = &self.data_str {
            return data_str.clone();
        }
        return default.to_string();
    }
}

pub struct Query<WindowState, AppState> {
    pub query: DomMessage,
    pub callback: Option<fn(&mut WindowState, QueryResponse)>,
    pub listener_callback: Option<Vec<fn(&mut WindowState, EventResponse, &mut App<AppState>)>>,
    pub listener_registered: bool,
    pub uid: i32,
}

pub struct QueryBuilder<WindowState, AppState> {
    queries: Vec<Query<WindowState, AppState>>,
    pub last: QueryResponse,
    last_timer_id: Option<i32>,
    ignore_timer: Vec<i32>,
}

impl<Window, AppState> QueryBuilder<Window, AppState> {
    pub fn new() -> Self {
        Self {
            queries: Vec::new(),
            last: QueryResponse::new(false),
            last_timer_id: None,
            ignore_timer: Vec::new(),
        }
    }

    pub fn import_css(&mut self, css: String, hot_reload: bool) -> &mut Self {
        self.build_query(&mut DomQueryResult::from_dom_message(DomMessage {
            message: DomInternalMessageType::ImportCss(css, hot_reload),
            uid: get_unique_id(),
            selector: DomQuery {
                query_type: DomQueryType::Unused,
                flag: None,
            },
        }));
        self
    }

    pub fn b(&mut self, e: &mut DomQueryResult) -> &mut Self {
        self.build_query(e)
    }

    pub fn build_query(&mut self, query_result: &mut DomQueryResult) -> &mut Self {
        if query_result.event.is_some() {
            let ev = query_result.event.as_ref().unwrap().clone();
            let id = get_unique_id();
            let query = Query {
                query: DomMessage {
                    message: ev.message,
                    uid: id,
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

    pub fn open_window(
        &mut self,
        window_params: Box<dyn Any>,
        app_state: &mut App<AppState>,
    ) -> &mut Self {
        app_state.open_window(window_params);
        self.set_timeout(0); // TODO: This is a hack to make sure the window is opened right away
        self
    }

    pub fn close_window(
        &mut self,
        window_id: WindowId,
        app_state: &mut App<AppState>,
    ) -> &mut Self {
        app_state.close_window(window_id);
        self.set_timeout(0); // TODO: Same as above
        self
    }

    pub fn set_timeout(&mut self, timeout: i32) -> &mut Self {
        self.last_timer_id = Some(get_unique_id());
        let dom_message = DomMessage {
            message: DomInternalMessageType::SubscribeDynamicEvent(DynamicEvent::SetTimeout(
                timeout,
            )),
            uid: self.last_timer_id.unwrap(),
            selector: DomQuery {
                query_type: DomQueryType::Unused,
                flag: None,
            },
        };
        self.build_query(&mut DomQueryResult::from_dom_message(dom_message));
        self
    }

    pub fn set_interval(&mut self, interval: i32) -> &mut Self {
        self.last_timer_id = Some(get_unique_id());
        let dom_message = DomMessage {
            message: DomInternalMessageType::SubscribeDynamicEvent(DynamicEvent::SetInterval(
                interval,
            )),
            uid: self.last_timer_id.unwrap(),
            selector: DomQuery {
                query_type: DomQueryType::Unused,
                flag: None,
            },
        };
        self.build_query(&mut DomQueryResult::from_dom_message(dom_message));
        self
    }

    pub fn get_timer_id(&mut self, id: &mut Option<i32>) -> &mut Self {
        *id = self.last_timer_id;
        self
    }

    pub fn clear_timer(&mut self, timer_id: i32) -> &mut Self {
        self.ignore_timer.push(timer_id);
        self
    }

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

    pub fn then(&mut self, callback: fn(&mut Window, QueryResponse)) -> &mut Self {
        if let Some(last_query) = self.queries.last_mut() {
            last_query.callback = Some(callback);
        }
        self
    }

    pub fn subscribe(&self, engine: &XmlEngine) -> Subscription<Message> {
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
        }
        return Subscription::batch(subscriptions);
    }

    pub fn fetch(
        &mut self,
        returned_callbacks: Vec<(i32, EventResponse)>,
    ) -> Vec<(
        fn(&mut Window, EventResponse, &mut App<AppState>),
        EventResponse,
    )> {
        let mut callbacks = Vec::new();
        for (uid, event_response) in returned_callbacks {
            if let Some(query) = self.queries.iter().find(|q| q.uid == uid) {
                if query.listener_callback.is_some() {
                    for callback in query.listener_callback.as_ref().unwrap().iter() {
                        callbacks.push((*callback, event_response.clone()));
                    }
                }
            }
        }
        return callbacks;
    }

    pub fn execute(
        &mut self,
        engine: &mut XmlEngine,
    ) -> Vec<(fn(&mut Window, QueryResponse), QueryResponse)> {
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
