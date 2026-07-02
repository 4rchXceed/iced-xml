use crate::dom::events::{DomQuery, DomQueryResult};

pub struct Dom {}

impl Dom {
    pub fn get_element<'a>(uid: i32) -> DomQueryResult {
        return DomQueryResult::new("uid".to_string(), uid.to_string());
    }

    pub fn get_element_by_id<'a>(id: &str) -> DomQueryResult {
        return DomQueryResult::new("id".to_string(), id.to_string());
    }

    pub fn get_elements_by_class<'a>(class: &str) -> DomQueryResult {
        return DomQueryResult::new("class".to_string(), class.to_string());
    }

    pub fn get_elements_by_tag<'a>(tag: &str) -> DomQueryResult {
        return DomQueryResult::new("tag".to_string(), tag.to_string());
    }

    pub fn all() -> DomQueryResult {
        return DomQueryResult::new("all".to_string(), "".to_string());
    }

    pub fn from(query: DomQuery) -> DomQueryResult {
        return DomQueryResult::from(query);
    }
}
