use crate::{
    dom::events::{DomInternalMessageType, DomMessage},
    xml_engine::XmlEngine,
};

#[derive(Debug, Clone)]
pub struct EventResponse {
    // HERE: All properties in Option<> for every event response, so that we can return None if the event is not applicable to the element
}

impl Default for EventResponse {
    fn default() -> Self {
        Self {}
    }
}

pub struct QueryResponse {
    pub success: bool,
}

pub struct Query<T> {
    pub query: DomMessage,
    pub callback: Option<fn(&mut T, QueryResponse)>,
    pub listener_callback: Option<fn(&mut T, EventResponse)>,
    pub uid: i32,
}

pub struct QueryBuilder<T> {
    queries: Vec<Query<T>>,
    current_uid: i32,
}

impl<T> QueryBuilder<T> {
    pub fn new() -> Self {
        Self {
            queries: Vec::new(),
            current_uid: 0,
        }
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
            uid: self.current_uid,
        };
        self.current_uid += 1;
        self.queries.push(query);
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
        for query in &self.queries {
            let response = engine.client_events(&query.query);
            if let Some(callback) = query.callback {
                callbacks.push((callback, response));
            }
            if query.listener_callback.is_none() {
                queries_to_remove.push(i);
            }
            i += 1;
        }
        for index in queries_to_remove.iter().rev() {
            self.queries.remove(*index);
        }
        return callbacks;
    }
}
