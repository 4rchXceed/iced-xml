use crate::{
    css_reader::{CssReader, Selector, split_complex_selector},
    dom::query::DomEvent,
    xml_engine::DynamicEvent,
    xml_struct::element_renderer::extract_selector_style_flag,
};

#[derive(Debug)]
pub enum ComplexQueryJoinType {
    Descendant, // " "
    Child,      // ">"
    Silbling,   // "~"
    Also,       // Tag#id.class
}

impl ComplexQueryJoinType {
    pub fn from(s: Option<char>) -> Option<Self> {
        if s.is_none() {
            return None;
        } else {
            return Some(match s.as_ref().unwrap() {
                ' ' => ComplexQueryJoinType::Descendant,
                '>' => ComplexQueryJoinType::Child,
                '~' => ComplexQueryJoinType::Silbling,
                '=' => ComplexQueryJoinType::Also,
                _ => panic!(
                    "Invalid complex query join type: {} [shouldn't happen]",
                    s.unwrap()
                ),
            });
        }
    }
}

// a b > c
// d ~ e

#[derive(Debug)]
pub struct ComplexQuery {
    pub query: DomQueryType,
    pub next: Option<Box<ComplexQuery>>,
    pub link_next: Option<ComplexQueryJoinType>,
}

impl ComplexQuery {
    pub fn new(mut query_part: String, link: Option<char>) -> Self {
        query_part.push_str(",");
        let query: Selector = CssReader::new(query_part.as_str()).parse_selector();
        let query_type = gen_query_type(query.selector_type, query.content);
        match query_type {
            DomQueryType::Complex(_) => {
                println!(
                    "Complex query type is not supported in ComplexQuery: {:?}",
                    query_type
                );
                return Self {
                    query: DomQueryType::Unused,
                    next: None,
                    link_next: ComplexQueryJoinType::from(link),
                };
            }
            _ => {}
        }
        return Self {
            query: query_type,
            next: None,
            link_next: ComplexQueryJoinType::from(link),
        };
    }

    fn next(mut full: Vec<(String, Option<char>)>) -> Self {
        let (query_part, link_next) = full.remove(0);
        let mut base = ComplexQuery::new(query_part, link_next);
        if full.len() > 0 {
            base.next = Some(Box::new(ComplexQuery::next(full)));
        }
        return base;
    }

    pub fn from(full_query: String) -> Self {
        let full = split_complex_selector(full_query);
        println!("Complex query parts: {:?}", full);
        return ComplexQuery::next(full);
    }
}

#[derive(Debug, Clone, Hash)]
pub enum DomQueryType {
    ById(String),
    ByUid(i32),
    Class(String),
    Tag(String),
    Complex(String),
    All,
    Unused,
}

#[derive(Debug, Clone, Hash)]
pub struct DomQuery {
    pub query_type: DomQueryType,
    pub flag: Option<String>,
}

pub fn gen_query_type(selector_type: String, val: String) -> DomQueryType {
    return match selector_type.as_str() {
        "id" => DomQueryType::ById(val),
        "uid" => DomQueryType::ByUid(val.parse::<i32>().unwrap()),
        "class" => DomQueryType::Class(val),
        "tag" => DomQueryType::Tag(val),
        "all" => DomQueryType::All,
        "complex" => DomQueryType::Complex(val),
        "none" => DomQueryType::Unused,
        _ => panic!("Invalid query type: {}", selector_type),
    };
}

impl DomQuery {
    pub fn new(selector_type: String, val: String, flag: Option<String>) -> Self {
        return Self {
            query_type: gen_query_type(selector_type, val),
            flag: flag,
        };
    }
}

#[derive(Debug, Clone, Hash)]
pub struct DomMessage {
    pub message: DomInternalMessageType,
    pub uid: i32,
    pub selector: DomQuery,
}

#[derive(Debug, Clone, Hash)]
pub enum DomInternalMessageType {
    StyleChange(String, String, Option<String>), // k => v custom_style_flag [for(xyz)]
    PropertyChange(String, String),              // k => v
    GetProperty(String),                         // key
    RegisterEventListener(String),               // event_name
    ImportCss(String, bool),                     // css content
    SubscribeDynamicEvent(DynamicEvent), // dynamic events (like set_timeout, set_interval, etc.)
    GetData(String),                     // key
    FireEvent(String, DomEvent),         // event name, event data
}

#[derive(Debug, Clone)]
pub struct DomQueryResult {
    query_event: DomQuery,
    pub event: Option<DomMessage>,
}

impl DomQueryResult {
    pub fn new(query_type: String, element: String) -> Self {
        Self {
            query_event: DomQuery::new(query_type, element, None),
            event: None,
        }
    }

    pub fn with_flag(&mut self, flag: String) -> &mut Self {
        let style_flag = extract_selector_style_flag(&flag);
        if style_flag.is_some() {
            if self.event.is_some() {
                self.event = Some(DomMessage {
                    message: match &self.event.as_ref().unwrap().message {
                        DomInternalMessageType::StyleChange(k, v, _) => {
                            DomInternalMessageType::StyleChange(
                                k.clone(),
                                v.clone(),
                                Some(style_flag.unwrap()),
                            )
                        }
                        _ => self.event.as_ref().unwrap().message.clone(),
                    },
                    uid: self.event.as_ref().unwrap().uid,
                    selector: self.event.as_ref().unwrap().selector.clone(),
                });
            }
        } else {
            self.query_event.flag = Some(flag);
        }
        return self;
    }

    pub fn from(dom_query: DomQuery) -> Self {
        Self {
            query_event: dom_query,
            event: None,
        }
    }

    pub fn from_dom_message(dom_message: DomMessage) -> Self {
        Self {
            query_event: dom_message.selector.clone(),
            event: Some(dom_message),
        }
    }

    pub fn set_property(&mut self, key: &str, value: &str) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::PropertyChange(key.to_string(), value.to_string()),
            uid: -1,
            selector: self.query_event.clone(),
        };
        self.event = Some(event);
        return self;
    }

    pub fn get_property(&mut self, key: &str) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::GetProperty(key.to_string()),
            uid: -1,
            selector: self.query_event.clone(),
        };
        self.event = Some(event);
        return self;
    }

    pub fn set_style(&mut self, key: &str, value: &str) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::StyleChange(key.to_string(), value.to_string(), None),
            uid: -1,
            selector: self.query_event.clone(),
        };
        self.event = Some(event);
        return self;
    }

    pub fn add_event_listener(&mut self, name: &str) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::RegisterEventListener(name.to_string()),
            uid: -1,
            selector: self.query_event.clone(),
        };
        self.event = Some(event);
        return self;
    }

    pub fn get_data(&mut self, key: &str) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::GetData(key.to_string()),
            uid: -1,
            selector: self.query_event.clone(),
        };
        self.event = Some(event);
        return self;
    }

    pub fn fire_event(&mut self, name: &str, data: &mut DomEvent) -> &mut Self {
        let event = DomMessage {
            message: DomInternalMessageType::FireEvent(name.to_string(), data.clone()),
            uid: -1,
            selector: self.query_event.clone(),
        };
        self.event = Some(event);
        return self;
    }
}
