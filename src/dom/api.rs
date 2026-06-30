use std::any::Any;

use crate::{
    dom::events::DomQueryResult,
    xml_engine::Message,
    xml_struct::elements::{AnyElement, ElementRenderer},
};

pub struct Dom {}

impl Dom {
    pub fn get_element<'a>(uid: i32) -> DomQueryResult {
        return DomQueryResult::new("uid".to_string(), uid.to_string());
    }

    pub fn get_element_by_id<'a>(id: &str) -> DomQueryResult {
        return DomQueryResult::new("id".to_string(), id.to_string());
    }
}
