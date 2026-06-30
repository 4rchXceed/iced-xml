use iced_xml::{
    app_wrapper::{AppTemplate, render, run_app},
    dom::query::QueryBuilder,
    xml_engine::{Message, XmlEngine},
};

const XML_FILE: &[u8; include_bytes!("main.xml").len()] = include_bytes!("main.xml");

struct App {
    engine: XmlEngine,
    qb: QueryBuilder<App>,
}

impl AppTemplate for App {
    fn new() -> Self {
        let engine = XmlEngine::new(XML_FILE.to_vec());
        let qb = QueryBuilder::new();
        Self { engine, qb }
    }

    fn render(&self) -> iced::Element<'_, Message> {
        return render(&self.engine);
    }

    fn process(&mut self) {
        for query in self.qb.execute(&mut self.engine) {
            (query.0)(self, query.1);
        }
    }

    fn update(&mut self, message: iced_xml::xml_engine::Message) {
        for query in self.qb.fetch(self.engine.update(message)) {
            (query.0)(self, query.1);
        }
    }
}

fn main() -> iced::Result {
    return run_app(App::new, App::update, App::render);
}
