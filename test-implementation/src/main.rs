use std::{path::Path, sync::mpsc};

use iced_xml::{
    app_wrapper::{AppTemplate, Objects, ObjectsReadOnly, run_app},
    dom::{
        api::Dom,
        query::{QueryBuilder, QueryResponse},
    },
    xml_engine::XmlEngine,
};
use notify::{Event, RecommendedWatcher, Result, Watcher};

const XML_FILE: &[u8; include_bytes!("main.xml").len()] = include_bytes!("main.xml");

struct App {
    engine: XmlEngine,
    qb: QueryBuilder<App>,
    is_dark: bool,
    #[allow(dead_code)]
    watcher: RecommendedWatcher, // Else the watcher will be dropped and stop watching
    rx_watcher: mpsc::Receiver<Result<Event>>,
}

impl App {
    fn fatal_selector(&mut self, result: QueryResponse) {
        if !result.success {
            panic!("Error selecting element :'(");
        }
    }

    fn load_file(&mut self, path: &str) -> String {
        let content = std::fs::read_to_string(path);
        if content.is_err() {
            panic!("Error loading file: {}", path);
        }
        return content.unwrap();
    }

    fn reload_css(&mut self, path: &str) {
        let content = self.load_file(path);
        self.qb.import_css(content, true).then(|_, d| {
            if !d.success {
                println!("Error: {}", d.error_message.unwrap_or_default());
            }
        });
    }
}

impl AppTemplate<Self> for App {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Result<Event>>();
        let watcher = notify::recommended_watcher(tx);
        if watcher.is_err() {
            panic!("Error creating file watcher: {:?}", watcher.err());
        }
        let mut watcher = watcher.unwrap();
        _ = watcher.watch(
            Path::new("src/style.css"),
            notify::RecursiveMode::NonRecursive,
        );

        return Self {
            engine: XmlEngine::new(XML_FILE.to_vec()),
            qb: QueryBuilder::new(),
            is_dark: true,
            rx_watcher: rx,
            watcher: watcher,
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
        self.reload_css("src/style.css");
        self.qb
            .b(Dom::get_element_by_id("hot-css-reload").add_event_listener("click"))
            .with_callback(|me, _| {
                me.reload_css("src/style.css");
                me.process();
            });
        self.qb.set_interval(100).with_callback(|me, _| {
            let res = me.rx_watcher.try_recv();
            match res {
                Ok(event) => match event.unwrap().kind {
                    notify::EventKind::Access(kind) => match kind {
                        notify::event::AccessKind::Close(op) => match op {
                            notify::event::AccessMode::Write => {
                                println!("File closed, reloading CSS...");
                                me.reload_css("src/style.css");
                                me.process();
                            }
                            _ => (),
                        },
                        _ => (),
                    },
                    _ => {}
                },
                Err(_) => (), // There's no event, just continue
            }
        });
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
            })
            .then(App::fatal_selector);
        self.qb
            .b(Dom::get_element_by_id("checkbox-01").add_event_listener("checked"))
            .with_callback(|me, _| {
                me.qb
                    .b(Dom::get_element_by_id("checkbox-01").get_property("checked"))
                    .then(|_, d| {
                        if d.success && d.data_bool.is_some() {
                            println!("Checkbox 01 is checked: {}", d.data_bool.unwrap());
                        }
                    });
                me.process();
            });
        self.process();
    }
}

fn main() -> iced::Result {
    return run_app::<App>();
}
