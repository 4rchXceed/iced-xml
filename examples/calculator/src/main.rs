use iced_xml::{
    app_wrapper::{AppResult, AppTemplate, CssRx, CssWatcher, Objects, ObjectsReadOnly, run_app},
    dom::{api::Dom, query::QueryBuilder},
    utils::watch_css::watch_css_file,
    xml_engine::XmlEngine,
};

struct App {
    qb: QueryBuilder<App>,
    engine: XmlEngine,
    rx: Option<CssRx>,
    watcher: Option<CssWatcher>,
    first_nbr: Option<String>,
    second_nbr: Option<String>,
    current: usize,
    result: f32,
    op: String,
}

impl App {
    fn add_nbr(&mut self, nbr: String) {
        if self.current == 0 {
            if self.first_nbr.is_none() {
                self.first_nbr = Some(String::from(""));
            }
            self.first_nbr.as_mut().unwrap().push_str(nbr.as_str());
        } else {
            if self.second_nbr.is_none() {
                self.second_nbr = Some(String::from(""));
            }
            self.second_nbr.as_mut().unwrap().push_str(nbr.as_str());
        }
        self.update_ui();
    }

    fn set_op(&mut self, op: String) {
        self.op = op;
        self.current = (self.current + 1) % 2;
        self.update_ui();
    }

    fn to_nbr(&self, nbr: &str) -> f32 {
        nbr.parse::<f32>().unwrap()
    }

    fn calculate(&mut self) {
        if self.first_nbr.is_none() || self.second_nbr.is_none() {
            self.result = 0.0;
        } else {
            let first = self.to_nbr(&self.first_nbr.as_ref().unwrap());
            let second = self.to_nbr(&self.second_nbr.as_ref().unwrap());
            self.result = match self.op.as_str() {
                "+" => first + second,
                "-" => first - second,
                "*" => first * second,
                "/" => first / second,
                _ => 0.0,
            };
        }
        self.update_ui();
    }

    fn update_ui(&mut self) {
        self.qb.b(Dom::get_element_by_id("label-result")
            .set_property("text", &self.result.to_string().as_str()));
        self.qb
            .b(Dom::get_element_by_id("label-second-nbr").set_property(
                "text",
                &self.second_nbr.as_ref().unwrap_or(&"0".to_string()),
            ));
        self.qb.b(Dom::get_element_by_id("label-first-nbr")
            .set_property("text", &self.first_nbr.as_ref().unwrap_or(&"0".to_string())));
        self.qb.b(Dom::get_element_by_id("label-operator")
            .set_property("text", &format!(" {} ", self.op)));
        self.process();
    }
}

impl AppTemplate<App> for App {
    fn get_objects(&mut self) -> Objects<'_, Self> {
        return Objects {
            engine: &mut self.engine,
            qb: &mut self.qb,
            css_watcher_rx: self.rx.as_mut(),
            css_path: "style.css",
        };
    }
    fn set_css_watcher_rx(&mut self, rx: CssRx) {
        self.rx = Some(rx);
    }
    fn get_objects_read_only(&self) -> iced_xml::app_wrapper::ObjectsReadOnly<'_, Self> {
        return ObjectsReadOnly {
            engine: &self.engine,
            qb: &self.qb,
        };
    }
    fn get_self(&mut self) -> &mut Self {
        return self;
    }
    fn new() -> Self {
        Self {
            qb: QueryBuilder::new(),
            engine: XmlEngine::new(include_bytes!("main.xml").to_vec()),
            rx: None,
            watcher: None,
            first_nbr: None,
            second_nbr: None,
            current: 0,
            result: 0.0,
            op: String::from("+"),
        }
    }

    fn post_construct(&mut self) {
        let watcher_result = watch_css_file(self, 100);
        if watcher_result.is_ok() {
            self.watcher = Some(watcher_result.unwrap());
        }
        self.qb
            .b(Dom::get_elements_by_class("button-nbr").add_event_listener("click"))
            .with_callback(|this, datas| {
                if datas.target.is_some() {
                    this.qb
                        .b(Dom::from(datas.target.unwrap()).get_property("text"));
                    this.process();
                    if this.qb.last.success {
                        this.add_nbr(this.qb.last.data_str.as_deref().unwrap().to_string());
                    }
                }
            });
        self.qb
            .b(Dom::get_elements_by_class("button-op").add_event_listener("click"))
            .with_callback(|this, datas| {
                if datas.target.is_some() {
                    this.qb.b(Dom::from(datas.target.unwrap()).get_data("op"));
                    this.process();
                    if this.qb.last.success {
                        this.set_op(this.qb.last.data_str.as_deref().unwrap().to_string());
                    }
                }
            });

        self.qb
            .b(Dom::get_element_by_id("button-calculate").add_event_listener("click"))
            .with_callback(|this, _| {
                this.calculate();
            });
        self.qb
            .b(Dom::get_element_by_id("button-clear").add_event_listener("click"))
            .with_callback(|this, _| {
                this.first_nbr = None;
                this.second_nbr = None;
                this.result = 0.0;
                this.current = 0;
                this.update_ui();
            });
        self.update_ui();
    }
}

fn main() -> AppResult {
    return run_app::<App>();
}
