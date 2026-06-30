use iced_xml::{
    app_wrapper::{AppTemplate, Objects, ObjectsReadOnly, run_app},
    dom::{
        api::Dom,
        query::{QueryBuilder, QueryResponse},
    },
    xml_engine::XmlEngine,
};

const XML_FILE: &[u8; include_bytes!("main.xml").len()] = include_bytes!("main.xml");

struct App {
    engine: XmlEngine,
    qb: QueryBuilder<App>,
    is_dark: bool,
}

impl App {
    fn fatal_selector(&mut self, result: QueryResponse) {
        if !result.success {
            panic!("Error selecting element :(");
        }
    }
}

impl AppTemplate<Self> for App {
    fn new() -> Self {
        return Self {
            engine: XmlEngine::new(XML_FILE.to_vec()),
            qb: QueryBuilder::new(),
            is_dark: true,
        };
    }
    fn get_objects(&mut self) -> Objects<'_, Self> {
        return Objects {
            engine: &mut self.engine,
            qb: &mut self.qb,
        };
    }
    fn get_objects_read_only(&self) -> ObjectsReadOnly<'_, Self> {
        return ObjectsReadOnly {
            engine: &self.engine,
            qb: &self.qb,
        };
    }
    fn get_self(&mut self) -> &mut Self {
        return self;
    }
    fn post_construct(&mut self) {
        self.qb
            .b(Dom::get_element_by_id("btn-1").add_event_listener("click"))
            .with_callback(|me, _| {
                me.qb
                    .b(Dom::get_element_by_id("btn-1").set_property("text", "Clicked!"));
                me.process();
            });
        self.qb
            .b(Dom::get_element_by_id("dark-mode").add_event_listener("click"))
            .with_callback(|me, _| {
                me.is_dark = !me.is_dark;
                let app_container = Dom::all();
                if me.is_dark {
                    me.qb.b(app_container.clone().set_style("fg", "white"));
                    me.qb.b(app_container.clone().set_style("bg", "black"));
                } else {
                    me.qb.b(app_container.clone().set_style("fg", "black"));
                    me.qb.b(app_container.clone().set_style("bg", "white"));
                }
                me.process();
            });
        self.process();
    }
}

fn main() -> iced::Result {
    return run_app::<App>();
}
