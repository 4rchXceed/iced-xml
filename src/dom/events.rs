#[derive(Debug, Clone)]
pub enum DomQueryEvent {
    ById(String),
    ByUid(i32),
}

pub enum DomInternalEvent {
    StyleChange(DomQueryEvent, String, String),    // k => v
    PropertyChange(DomQueryEvent, String, String), // k => v
}

pub struct DomQueryResult {
    query_event: DomQueryEvent,
}

impl DomQueryResult {
    pub fn new(query_type: String, element: String) -> Self {
        Self {
            query_event: match query_type.as_str() {
                "id" => DomQueryEvent::ById(element),
                "uid" => DomQueryEvent::ByUid(element.parse::<i32>().unwrap()),
                _ => panic!("Invalid query type"),
            },
        }
    }

    pub fn set_property(&mut self, key: &str, value: &str) -> DomInternalEvent {
        let event = DomInternalEvent::PropertyChange(
            self.query_event.clone(),
            key.to_string(),
            value.to_string(),
        );
        return event;
    }

    pub fn set_style(&mut self, key: &str, value: &str) -> DomInternalEvent {
        let event = DomInternalEvent::StyleChange(
            self.query_event.clone(),
            key.to_string(),
            value.to_string(),
        );
        return event;
    }

    pub fn add_event_listener<T>(&mut self, name: String, callback: fn(&mut T)) {}
}
