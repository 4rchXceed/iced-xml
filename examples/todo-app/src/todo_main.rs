use crate::{DEFAULT_ENGINE_SETTINGS, state::AppState};
use iced_xml::{
    dom::{query_builder::QueryBuilder, types::QueryResponse},
    xml_engine::XmlEngine,
};
use iced_xml_macros::window;

#[window(AppState)]
pub struct TodoListWindow {}

impl TodoListWindow {
    pub fn new() -> Self {
        let qb = QueryBuilder::new();
        let engine = XmlEngine::new(
            include_str!("./todo_main.xml").to_string(),
            DEFAULT_ENGINE_SETTINGS,
        );

        Self { qb, engine }
    }

    pub fn init(&mut self) {}

    pub fn log_on_error(&mut self, qr: QueryResponse) {
        if !qr.success
            && let Some(e) = qr.error_message
        {
            eprintln!("Error in DOM manipulation: {e}");
        }
    }
}
