use crate::{
    DEFAULT_ENGINE_SETTINGS,
    params::{WindowParams, WindowType},
    state::AppState,
    task::Task,
};
use iced_xml::{
    app_manager::state_wrapper::App,
    dom::{
        api::Dom,
        query_builder::QueryBuilder,
        types::{EventListenerTypes, EventResponse, QueryResponse},
    },
    window_wrapper::WindowTemplate,
    xml_engine::XmlEngine,
    xml_struct::{parser::xml, xml_element::XmlElement},
};
use iced_xml_macros::window;

#[window(AppState)]
pub struct TodoListWindow {}

impl TodoListWindow {
    pub fn new() -> Self {
        let qb = QueryBuilder::new();
        let engine = XmlEngine::new(
            include_str!("./todo_main.xml").to_string(),
            DEFAULT_ENGINE_SETTINGS.clone(),
        );

        Self { qb, engine }
    }

    pub fn init(&mut self, app: &mut App<AppState>) {
        self.qb
            .b(
                Dom::get_element_by_id("new-todo-btn")
                    .add_event_listener(EventListenerTypes::Click),
            )
            .with_callback(|this, _, app| {
                this.add_task(app);
            })
            .then(Self::log_on_error);

        self.reload_tasks(app);
    }

    fn log_on_error(&mut self, qr: QueryResponse) {
        if !qr.success
            && let Some(e) = qr.error_message
        {
            eprintln!("Error in DOM manipulation: {e}");
        }
    }

    fn add_task(&mut self, app: &mut App<AppState>) {
        self.qb
            .b(Dom::get_element_by_id("new-todo-name").get_property("value"))
            .then(Self::log_on_error);

        let name_result = self.process();

        self.qb
            .b(Dom::get_element_by_id("new-todo-description").get_property("value"))
            .then(Self::log_on_error);

        let description_result = self.process();

        if let Some(name) = name_result.data_str
            && let Some(description) = description_result.data_str
        {
            if !name.is_empty() && !description.is_empty() {
                app.state.tasks.push(Task {
                    done: false,
                    name: name,
                    description: description,
                });

                self.reload_tasks(app);
            }
        }
    }

    fn reload_tasks(&mut self, app: &mut App<AppState>) {
        let mut tasks_container = XmlElement::void();
        tasks_container.set_tag("Col");
        tasks_container.set_id("tasks-list");

        for (id, task) in app.state.tasks.iter().enumerate() {
            let mut task_element = XmlElement::void();
            task_element.set_tag("Row");

            let mut task_title = XmlElement::void();
            task_title.set_tag("Label");
            task_title.set_text(&format!(
                "{}: {}",
                task.name,
                if task.done { "Done" } else { "To do" }
            ));

            let mut task_details_btn = xml(
                "<Button classes=\"view-more-btn\" style:border-width=\"1\" style:border-color=\"black\">View more</Button>",
            );
            task_details_btn.add_data("taskid", &id.to_string());

            let mut task_done_btn = xml(
                "<Button classes=\"done-btn\" style:border-width=\"1\" style:border-color=\"black\">Done</Button>",
            );
            task_done_btn.add_data("taskid", &id.to_string());

            task_element.append_child(task_title);
            task_element.append_child(task_details_btn);
            task_element.append_child(task_done_btn);

            tasks_container.append_child(task_element);
        }

        self.qb
            .b(Dom::get_element_by_id("tasks-list").replace(Box::new(tasks_container)))
            .then(Self::log_on_error);

        self.qb
            .b(
                Dom::get_elements_by_class("done-btn")
                    .add_event_listener(EventListenerTypes::Click),
            )
            .with_callback(Self::on_done_click)
            .then(Self::log_on_error);

        self.qb
            .b(Dom::get_elements_by_class("view-more-btn")
                .add_event_listener(EventListenerTypes::Click))
            .with_callback(Self::on_view_more_click)
            .then(Self::log_on_error);

        self.process();
    }

    fn on_done_click(&mut self, er: EventResponse, app: &mut App<AppState>) {
        if let Some(selector) = er.target {
            self.qb.b(Dom::from(selector).get_data("taskid"));

            let result = self.process();

            if let Some(id_str) = result.data_str
                && let Ok(id) = id_str.parse::<usize>()
            {
                if let Some(task) = app.state.tasks.get_mut(id) {
                    task.done = true;

                    self.reload_tasks(app);
                }
            }
        }
    }

    fn on_view_more_click(&mut self, er: EventResponse, app: &mut App<AppState>) {
        if let Some(selector) = er.target {
            self.qb.b(Dom::from(selector).get_data("taskid"));

            let result = self.process();

            if let Some(id_str) = result.data_str
                && let Ok(id) = id_str.parse::<usize>()
            {
                self.qb.open_window(
                    Box::new(WindowParams {
                        window: WindowType::TodoDetailWindow(id),
                    }),
                    app,
                );

                self.process();
            }
        }
    }
}
