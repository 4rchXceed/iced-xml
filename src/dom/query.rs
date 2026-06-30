use crate::{dom::events::DomInternalEvent, xml_engine::XmlEngine};

pub struct QueryResponse {
    pub success: bool,
}

pub struct Query<T> {
    pub query: DomInternalEvent,
    pub callback: Option<fn(&mut T, QueryResponse)>,
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

    pub fn b(&mut self, e: DomInternalEvent) -> &mut Self {
        self.build_query(e)
    }

    pub fn build_query(&mut self, event: DomInternalEvent) -> &mut Self {
        let query = Query {
            query: event,
            callback: None,
        };
        self.queries.push(query);
        self
    }

    pub fn then(&mut self, callback: fn(&mut T, QueryResponse)) -> &mut Self {
        if let Some(last_query) = self.queries.last_mut() {
            last_query.callback = Some(callback);
        }
        self
    }

    pub fn execute(
        &mut self,
        engine: &mut XmlEngine,
    ) -> Vec<(fn(&mut T, QueryResponse), QueryResponse)> {
        let mut callbacks = Vec::new();
        while let query = self.queries.pop()
            && query.is_some()
        {
            let query = query.unwrap();
            let response = engine.client_events(query.query);
            if let Some(callback) = query.callback {
                callbacks.push((callback, response));
            }
        }
        return callbacks;
    }
}
