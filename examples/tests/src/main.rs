use std::collections::HashMap;

use iced_xml::{
    app_manager::{app_manager::WindowManager, state_wrapper::App, utils::WindowId},
    dom::{
        api::Dom,
        events::EventListenerTypes,
        query_builder::{CustomElementEvent, QueryBuilder},
    },
    rs_utils::HashableHashMap,
    utils::watch_css::watch_css_file,
    window_manager,
    window_wrapper::{AppResult, CssRx, Objects, ObjectsReadOnly, WindowTemplate},
    xml_engine::XmlEngine,
    xml_struct::elements::textarea::TextareaEvent,
};

#[derive(Debug)]
struct VocaSynth {
    name: String,
    provider: String,
}

impl VocaSynth {
    pub fn into_hashmap(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(String::from("name"), self.name.clone());
        map.insert(String::from("provider"), self.provider.clone());
        return map;
    }
}

pub struct AppState {}

#[derive(Clone)]
struct WindowParams {}

pub struct MainWindow {
    qb: QueryBuilder<MainWindow, AppState>,
    engine: XmlEngine,
    test01_counter: u32,
    test_multiple_event_listeners_counter: u32,
    progress_val: f32,
    progress_multiplier: i32,
    voca_synths: Vec<VocaSynth>,
    css_watcher_rx: Option<CssRx>,
}

impl MainWindow {
    pub fn new() -> Self {
        return Self {
            qb: QueryBuilder::new(),
            engine: XmlEngine::new(
                String::from(include_str!("main.xml")),
                DEFAULT_ENGINE_SETTINGS.clone(),
            ),
            test01_counter: 0,
            test_multiple_event_listeners_counter: 0,
            progress_val: 0.0,
            progress_multiplier: 1,
            voca_synths: vec![VocaSynth {
                name: String::from("Hatsune Miku"),
                provider: String::from("Vocaloid"),
            }],
            css_watcher_rx: None,
        };
    }

    pub fn ok(&mut self, test_id: &str) {
        println!("Test {} OK", test_id);
        self.qb
            .b(Dom::get_element_by_id(test_id).set_property("text", "OK"));
        self.qb
            .b(Dom::get_element_by_id(test_id).set_style("fg", "rgb(0, 255, 0)"));
        self.process();
    }

    pub fn fail(&mut self, test_id: &str) {
        println!("Test {} FAIL", test_id);
        self.qb
            .b(Dom::get_element_by_id(test_id).set_property("text", "FAIL"));
        self.qb
            .b(Dom::get_element_by_id(test_id).set_style("fg", "red"));
        self.process();
    }

    pub fn first_test(&mut self) {
        self.qb
            .b(Dom::get_element_by_id("test01-subject").get_data("value"))
            .then(|this, res| {
                this.qb.b(res
                    .selector()
                    .unwrap()
                    .set_property("text", &res.data_str.unwrap()));
                this.process();
            });
        self.process();
    }

    pub fn first_check(&mut self) {
        self.qb
            .b(&mut Dom::get_element_by_id("test01-subject").get_property("text"));
        let res = self.process();
        if res.data_str == Some(String::from("test01-newtext")) {
            self.ok("test01-result");
        } else {
            self.fail("test01-result");
        }
    }

    pub fn second_check(&mut self) {
        self.qb
            .b(&mut Dom::get_element_by_id("test02-subject")
                .add_event_listener(EventListenerTypes::Check))
            .with_callback(|this, datas, _| {
                this.test01_counter += 1;
                if this.test01_counter == 1 {
                    if datas.data_bool != Some(true) {
                        this.fail("test02-result");
                    }
                }
                if this.test01_counter == 2 {
                    if datas.data_bool == Some(false) {
                        this.ok("test02-result");
                    } else {
                        this.fail("test02-result");
                    }
                }
            });
        self.process();
    }

    fn third_check(&mut self) {
        self.qb
            .b(&mut Dom::get_element_by_id("test03-subject")
                .add_event_listener(EventListenerTypes::Click))
            .with_callback(|this, _, _| {
                this.ok("test03-result");
            });
        self.process();
    }

    fn test_multiple_event_listeners(&mut self) {
        self.qb
            .b(&mut Dom::get_element_by_id("test04-subject")
                .add_event_listener(EventListenerTypes::Click))
            .with_callback(|this, _, _| {
                this.test_multiple_event_listeners_counter += 1;
                if this.test_multiple_event_listeners_counter == 2 {
                    this.ok("test04-result");
                }
            });
        self.qb
            .b(&mut Dom::get_element_by_id("test04-subject")
                .add_event_listener(EventListenerTypes::Click))
            .with_callback(|this, _, _| {
                this.test_multiple_event_listeners_counter += 1;
                if this.test_multiple_event_listeners_counter == 2 {
                    this.ok("test04-result");
                }
            });
        self.process();
    }

    pub fn check_input(&mut self) {
        self.qb
            .b(Dom::get_element_by_id("test06-subject")
                .add_event_listener(EventListenerTypes::Input))
            .with_callback(|this, datas, _| {
                if datas.data_str == Some(String::from("ok")) {
                    this.ok("test06-result");
                }
            });
        self.process();
    }

    pub fn run_progress(&mut self) {
        self.qb.set_interval(10).with_callback(|this, _, _| {
            if this.progress_val >= 100.0 {
                this.progress_multiplier = -1;
            }
            if this.progress_val <= 0.0 {
                this.progress_multiplier = 1;
            }
            this.progress_val += 1.0 * this.progress_multiplier as f32;
            this.qb.b(Dom::get_element_by_id("progress")
                .set_property("value", &this.progress_val.to_string()));
            this.process();
        });
        self.process();
    }

    pub fn check_radio(&mut self) {
        self.qb
            .b(Dom::get_elements_by_class("radio-group-1")
                .add_event_listener(EventListenerTypes::Select))
            .with_callback(|this, data, _| {
                if data.data_str == Some(String::from("2")) {
                    this.ok("test07-result");
                } else {
                    this.fail("test07-result");
                }
            });
        self.process();
    }

    pub fn check_range(&mut self) {
        self.qb
            .b(Dom::get_element_by_id("test08-subject")
                .add_event_listener(EventListenerTypes::Input))
            .with_callback(|this, callback, _| {
                if callback.data_float.map(|v| v.value()) == Some(100.0) {
                    this.ok("test08-result");
                }
            });
        self.process();
    }

    pub fn check_select(&mut self) {
        self.qb
            .b(Dom::get_element_by_id("test09-subject")
                .add_event_listener(EventListenerTypes::Select))
            .with_callback(|this, callback, _| {
                if callback.data_str == Some(String::from("2")) {
                    this.ok("test09-result");
                } else {
                    this.fail("test09-result");
                }
            });
    }

    pub fn update_vsynths(&mut self) {
        self.qb.b(Dom::get_element_by_id("vsynth-table").fire_event(
            CustomElementEvent::SetTableData(
                self.voca_synths
                    .iter()
                    .map(|v| HashableHashMap::new(v.into_hashmap()))
                    .collect::<Vec<HashableHashMap<String, String>>>(),
            ),
        ));
        self.process();
    }

    pub fn table_handler(&mut self) {
        self.qb
            .b(Dom::get_element_by_id("add-vsynth-btn")
                .add_event_listener(EventListenerTypes::Click))
            .with_callback(|this, _, _| {
                this.qb
                    .b(Dom::get_element_by_id("add-vsynth").get_property("value"));
                let qr = this.process();
                if qr.data_str.is_some() {
                    let val = qr.data_str.unwrap();
                    let parts: Vec<&str> = val.split(',').collect();
                    if parts.len() == 2 {
                        this.voca_synths.push(VocaSynth {
                            name: parts[0].to_string(),
                            provider: parts[1].to_string(),
                        });
                        this.update_vsynths();
                    }
                }
            });
        self.process();
    }

    pub fn textarea_test(&mut self) {
        self.qb
            .b(Dom::get_element_by_id("test10-subject")
                .add_event_listener(EventListenerTypes::TextareaEvent))
            .with_callback(|this, datas, _| {
                match datas.textarea_event.unwrap() {
                    TextareaEvent::Input((_, c)) => {
                        this.qb
                            .b(Dom::get_element_by_id("test10-subject").get_property("value"));
                        let last = this.process();
                        if last.data_str == Some(String::from("ok")) {
                            this.ok("test10-result");
                        } else if c.len() == 2 {
                            this.fail("test10-result");
                        }
                    }
                    _ => {}
                };
            });
        self.process();
    }

    pub fn post_construct(&mut self) {
        // self.qb
        //     .import_css(include_str!("style.css").to_string(), false);
        watch_css_file(self, 1000).expect("Failed to watch");
        self.qb
            .b(Dom::get_element_by_id("ui-debug").add_event_listener(EventListenerTypes::Click))
            .with_callback(|this, _, _| {
                this.qb.b(Dom::all().set_style("border-width", "1"));
                this.qb.b(Dom::all().set_style("border-color", "red"));
                this.process();
            });
        self.process();
        self.first_test();
        self.first_check();
        self.second_check();
        self.third_check();
        self.test_multiple_event_listeners();
        self.check_input();
        self.run_progress();
        self.check_radio();
        self.check_range();
        self.check_select();
        self.table_handler();
        self.update_vsynths();
        self.textarea_test();
    }
}

impl WindowTemplate<MainWindow, AppState> for MainWindow {
    fn get_objects(&mut self) -> Objects<'_, MainWindow, AppState> {
        return Objects {
            engine: &mut self.engine,
            qb: &mut self.qb,
            css_watcher_rx: self.css_watcher_rx.as_mut(),
            css_path: "src/style.css",
        };
    }

    fn set_css_watcher_rx(&mut self, rx: CssRx) {
        self.css_watcher_rx = Some(rx);
    }

    fn get_objects_read_only(&self) -> ObjectsReadOnly<'_, MainWindow, AppState> {
        return ObjectsReadOnly {
            engine: &self.engine,
            qb: &self.qb,
        };
    }

    fn get_self(&mut self) -> &mut MainWindow {
        return self;
    }

    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        return Box::new(self);
    }
}

fn create_window(_: WindowParams, _: &mut App<AppState>, _: WindowId) -> Windows {
    let mut window = MainWindow::new();
    window.post_construct();
    return Windows::MainWindow(window);
}

window_manager!(Windows {
    MainWindow
}; AppState; WindowParams);

pub fn main() -> AppResult {
    return WindowManager::run_app(
        WindowParams {},
        || AppState {},
        DEFAULT_ENGINE_SETTINGS.clone(),
        create_window,
    );
}
