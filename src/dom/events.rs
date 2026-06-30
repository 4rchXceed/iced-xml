#[derive(Debug, Clone)]
pub enum DomQuery {
    ById(String),
    ByUid(i32),
}

pub struct DomMessage {
    pub message: DomInternalMessageType,
    pub uid: i32,
    pub selector: DomQuery,
}

pub enum DomInternalMessageType {
    StyleChange(String, String),    // k => v
    PropertyChange(String, String), // k => v
    RegisterEventListener(String),  // event_name
}

pub struct DomQueryResult {
    query_event: DomQuery,
}

impl DomQueryResult {
    pub fn new(query_type: String, element: String) -> Self {
        Self {
            query_event: match query_type.as_str() {
                "id" => DomQuery::ById(element),
                "uid" => DomQuery::ByUid(element.parse::<i32>().unwrap()),
                _ => panic!("Invalid query type"),
            },
        }
    }

    pub fn set_property(&mut self, key: &str, value: &str) -> DomMessage {
        let event = DomMessage {
            message: DomInternalMessageType::PropertyChange(key.to_string(), value.to_string()),
            uid: -1,
            selector: self.query_event.clone(),
        };
        return event;
    }

    pub fn set_style(&mut self, key: &str, value: &str) -> DomMessage {
        let event = DomMessage {
            message: DomInternalMessageType::StyleChange(key.to_string(), value.to_string()),
            uid: -1,
            selector: self.query_event.clone(),
        };
        return event;
    }

    pub fn add_event_listener(&mut self, name: &str) -> DomMessage {
        let event = DomMessage {
            message: DomInternalMessageType::RegisterEventListener(name.to_string()),
            uid: -1,
            selector: self.query_event.clone(),
        };
        return event;
    }
}
