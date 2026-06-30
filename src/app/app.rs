use crate::{
    dom::{
        api::Dom,
        query::{QueryBuilder, QueryResponse},
    },
    xml_engine::XmlEngine,
};

pub struct UserApp {
    engine: XmlEngine,
    qb: QueryBuilder<UserApp>,
}

impl UserApp {
    pub fn new() -> Self {
        let engine = XmlEngine::new();
        Self {
            engine: engine,
            qb: QueryBuilder::new(),
        }
    }

    pub fn init(&mut self) {
        // self.engine
        //     .window
        //     .dom
        //     .get_element_by_id("btn-click-me")
        //     .unwrap()
        //     .add_event_listener("click".to_string(), AppState::activate);
        self.qb
            .b(Dom::get_element_by_id("main-lbl").set_style("fg", "blue"))
            .then(UserApp::fatal_query);
        self.process();
    }

    pub fn fatal_query(&mut self, response: QueryResponse) {
        if !response.success {
            panic!("Fatal query failed");
        }
    }

    pub fn update(&mut self, message: crate::xml_engine::Message) {
        self.engine.update(message);
    }

    pub fn render(&self) -> iced::Element<'_, crate::xml_engine::Message> {
        return self.engine.view();
    }

    pub fn process(&mut self) {
        for callback in self.qb.execute(&mut self.engine) {
            let (callback_fn, response) = callback;
            callback_fn(self, response);
        }
    }
}
