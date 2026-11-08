use crate::{DEFAULT_ENGINE_SETTINGS, state::AppState};
use iced_xml::{
    app_manager::state_wrapper::App,
    dom::{api::Dom, query_builder::QueryBuilder, types::QueryResponse},
    window_wrapper::WindowTemplate,
    xml_engine::XmlEngine,
};
use iced_xml_macros::window;

#[window(AppState)]
pub struct TodoDetailWindow {
    pub task_id: usize,
}

impl TodoDetailWindow {
    pub fn new(task_id: usize) -> Self {
        let qb = QueryBuilder::new();
        let engine = XmlEngine::new(
            include_str!("./todo_detail.xml").to_string(),
            DEFAULT_ENGINE_SETTINGS.clone(),
        );

        Self {
            qb,
            engine,
            task_id,
        }
    }

    fn log_on_error(&mut self, qr: QueryResponse) {
        if !qr.success
            && let Some(e) = qr.error_message
        {
            eprintln!("Error in DOM manipulation: {e}");
        }
    }

    pub fn init(&mut self, app: &mut App<AppState>) {
        let name = app
            .state
            .tasks
            .get(self.task_id)
            .map(|t| t.name.clone())
            .unwrap_or(String::from("Task not found"));

        let description = app
            .state
            .tasks
            .get(self.task_id)
            .map(|t| t.description.clone())
            .unwrap_or(String::new());

        self.qb
            .b(Dom::get_element_by_id("name").set_property("text", &format!("Name: {name}")))
            .then(Self::log_on_error);
        self.qb
            .b(Dom::get_element_by_id("description")
                .set_property("text", &format!("Description: {description}")))
            .then(Self::log_on_error);

        self.process();
    }
}
