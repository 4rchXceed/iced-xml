use crate::dom::events::DomQueryResult;

pub struct Dom {}

impl Dom {
    pub fn get_element<'a>(uid: i32) -> DomQueryResult {
        return DomQueryResult::new("uid".to_string(), uid.to_string());
    }

    pub fn get_element_by_id<'a>(id: &str) -> DomQueryResult {
        return DomQueryResult::new("id".to_string(), id.to_string());
    }

    pub fn get_element_by_class<'a>(class: &str) -> DomQueryResult {
        return DomQueryResult::new("class".to_string(), class.to_string());
    }

    pub fn all() -> DomQueryResult {
        return DomQueryResult::new("all".to_string(), "".to_string());
    }
}
