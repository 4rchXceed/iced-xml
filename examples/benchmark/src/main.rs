use std::process::exit;

#[cfg(feature = "dev-mode")]
use iced_xml::utils::watch_css::watch_css_file;
use iced_xml::window_manager;
#[cfg(feature = "dev-mode")]
use iced_xml::window_wrapper::CssRx;
#[cfg(feature = "dev-mode")]
use iced_xml::window_wrapper::CssWatcher;

use iced_xml::window_wrapper::WindowTemplate;
use iced_xml::{
    dom::{api::Dom, query::QueryBuilder},
    window_wrapper::{AppResult, Objects, ObjectsReadOnly},
    xml_engine::XmlEngine,
};

#[derive(Clone)]
struct WindowParams {}
#[derive(Clone)]
struct AppState {}

struct BenchmarkConfig {
    iterations: i32,
}

struct BenchmarkMainWindow {
    qb: QueryBuilder<BenchmarkMainWindow, AppState>,
    engine: XmlEngine,
    #[cfg(feature = "dev-mode")]
    css_rx: Option<CssRx>,
    #[allow(dead_code)]
    #[cfg(feature = "dev-mode")]
    _keep_watcher: Option<CssWatcher>,
    i: i32,
    start: std::time::Instant,
    benchmark_conig: BenchmarkConfig,
}

impl BenchmarkMainWindow {
    #[cfg(feature = "dev-mode")]
    fn watch_css(&mut self) {
        let watcher = watch_css_file(self, 1000);
        if watcher.is_ok() {
            self._keep_watcher = Some(watcher.unwrap());
        }
    }
    fn start_benchmark_set_timeout(&mut self) {
        println!(
            "Starting benchmark with {} iterations",
            self.benchmark_conig.iterations
        );
        // First: setInterval
        self.start = std::time::Instant::now();
        self.qb.set_interval(1).with_callback(|this, _, _| {
            this.i += 1;
            if this.i >= this.benchmark_conig.iterations {
                println!("Benchmark finished after {} iterations", this.i);
                let elapsed = this.start.elapsed();
                println!(
                    "Duration of {}ns. Perfect score would be 1'000'000'000.",
                    elapsed.as_nanos()
                );
                this.start = std::time::Instant::now();
                this.qb.set_interval(0).with_callback(|this, _, _| {
                    this.i += 1;
                    if this.i >= this.benchmark_conig.iterations {
                        println!("Benchmark finished after {}ns iterations", this.i);
                        let elapsed = this.start.elapsed();
                        println!(
                            "Total time: {}ns Perfect score would be 0.",
                            elapsed.as_nanos()
                        );
                        exit(0);
                    }
                });
                this.process();
            }
        });
        self.process();
    }
    fn start_benchmark_change_text(&mut self) {
        println!(
            "Starting benchmark with {} iterations",
            self.benchmark_conig.iterations
        );
        let f = std::time::Instant::now();
        self.qb
            .b(Dom::get_element_by_id("txt").set_property("text", "Benchmarking..."));
        self.process();
        let elapsed = f.elapsed();
        println!(
            "Time for 1 text change: {}ns. Perfect would be 0",
            elapsed.as_nanos()
        );
        // First: setInterval
        self.start = std::time::Instant::now();
        while self.i < self.benchmark_conig.iterations {
            self.i += 1;
            self.qb
                .b(Dom::get_element_by_id("txt").set_property("text", &self.i.to_string()));
            self.process();
        }
        println!("Benchmark finished after {}ns iterations", self.i);
        let elapsed = self.start.elapsed();
        println!(
            "duration of {:128}. Perfect score would be 0.",
            elapsed.as_nanos()
        );
        self.i = 0;
        self.start = std::time::Instant::now();
        self.qb.set_interval(0).with_callback(|this, _, _| {
            this.i += 1;
            this.qb
                .b(Dom::get_element_by_id("txt").set_property("text", &this.i.to_string()));
            this.process();
            if this.i >= this.benchmark_conig.iterations {
                println!("Benchmark finished after {}ns iterations", this.i);
                let elapsed = this.start.elapsed();
                println!(
                    "Changing text & reloading the page took {}ns. Perfect score would be 0.",
                    elapsed.as_nanos()
                );
                exit(0);
            }
        });
        self.process();
    }

    fn start_benchmark_styling(&mut self) {
        println!(
            "Starting benchmark with {} iterations",
            self.benchmark_conig.iterations
        );
        self.qb
            .b(Dom::get_element_by_id("txt").set_style("bg", "white"));
        self.process();
        let f = std::time::Instant::now();
        self.qb
            .b(Dom::get_element_by_id("txt").set_style("bg", "black"));
        self.process();
        let elapsed = f.elapsed();
        println!(
            "Time for 1 text change: {}ns. Perfect would be 0",
            elapsed.as_nanos()
        );
        // First: setInterval
        self.start = std::time::Instant::now();
        while self.i < self.benchmark_conig.iterations {
            self.i += 1;
            self.qb.b(Dom::get_element_by_id("container-styling")
                .set_style("bg", if self.i % 2 == 0 { "black" } else { "white" }));
            self.process();
        }
        println!("Benchmark finished after {}ns iterations", self.i);
        let elapsed = self.start.elapsed();
        println!(
            "duration of {:128}. Perfect score would be 0.",
            elapsed.as_nanos()
        );
        self.i = 0;
        self.start = std::time::Instant::now();
        self.qb.set_interval(0).with_callback(|this, _, _| {
            this.i += 1;
            this.qb.b(Dom::get_element_by_id("container-styling")
                .set_style("bg", if this.i % 2 == 0 { "black" } else { "white" }));
            this.process();
            if this.i >= this.benchmark_conig.iterations {
                println!("Benchmark finished after {}ns iterations", this.i);
                let elapsed = this.start.elapsed();
                println!(
                    "Changing text & reloading the page took {}ns. Perfect score would be 0.",
                    elapsed.as_nanos()
                );
                exit(0);
            }
        });
        self.process();
    }
    fn new() -> Self {
        let mut xml_content = std::fs::read("src/res/main.xml")
            .unwrap_or_else(|_| panic!("Failed to read XML file: src/res/main.xml"));
        let benchmark_type = std::env::args().nth(1).unwrap_or_else(|| {
            panic!("Please provide a benchmark type as the first argument. Options: set-timeout, change-text, style, lot-element")
        });
        let nbr_iterations = std::env::args().nth(2).unwrap_or_else(|| {
            panic!("Please provide the number of iterations as the second argument.")
        });
        let it = nbr_iterations
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Failed to parse number of iterations: {}", nbr_iterations));
        match benchmark_type.as_str() {
            "set-timeout" => {
                println!("Benchmarking set-timeout...");
            }
            "change-text" => {
                println!("Benchmarking change-text...");
            }
            "style" => {
                println!("Benchmarking style...");
                println!(
                    "This benchmark will change the background of an element very quickly, please make sure you are not sensitive to flashing lights. Do you want to continue? (y/n)"
                );
                let mut yesno_buffer = String::new();
                std::io::stdin().read_line(&mut yesno_buffer).unwrap();
                if yesno_buffer.trim() != "y" {
                    exit(0);
                }
                println!("Benchmarking style...");
            }
            "lot-element" => {
                println!("Benchmarking lot-element...");
                let mut content: String = String::from(
                    "<Window><Label style:font-size=\"10\">Lots of elements benchmark</Label><Row>",
                );
                for i in 0..it {
                    if i % 100 == 0 {
                        content.push_str("</Row><Row>");
                    }
                    content.push_str(&format!(
                        "<Label style:width=\"min\" style:height=\"min\" id=\"elem-{}\" style:font-size=\"3\">Element {}</Label>",
                        i,i
                    ));
                }
                content.push_str("</Row></Window>");
                xml_content = content.into_bytes();
            }
            _ => {
                panic!(
                    "Unknown benchmark type: {}. Options: set-timeout, change-text, style",
                    benchmark_type
                );
            }
        }
        Self {
            qb: QueryBuilder::new(),
            engine: XmlEngine::new(xml_content),
            #[cfg(feature = "dev-mode")]
            css_rx: None,
            #[cfg(feature = "dev-mode")]
            _keep_watcher: None,
            i: 0,
            benchmark_conig: BenchmarkConfig { iterations: it },
            start: std::time::Instant::now(),
        }
    }
}

impl WindowTemplate<BenchmarkMainWindow, AppState> for BenchmarkMainWindow {
    fn get_objects(&mut self) -> Objects<'_, Self, AppState> {
        return Objects {
            engine: &mut self.engine,
            qb: &mut self.qb,
            #[cfg(feature = "dev-mode")]
            css_watcher_rx: self.css_rx.as_mut(),
            #[cfg(feature = "dev-mode")]
            css_path: "src/res/style.css",
        };
    }

    fn get_objects_read_only(&self) -> ObjectsReadOnly<'_, Self, AppState> {
        return ObjectsReadOnly {
            engine: &self.engine,
            qb: &self.qb,
        };
    }

    fn get_self(&mut self) -> &mut Self {
        return self;
    }

    #[cfg(feature = "dev-mode")]
    fn set_css_watcher_rx(&mut self, rx: CssRx) {
        self.css_rx = Some(rx);
    }
}
impl BenchmarkMainWindow {
    fn post_construct(&mut self) {
        #[cfg(feature = "dev-mode")]
        self.watch_css();
        let benchmark_type = std::env::args().nth(1).unwrap_or_else(|| {
            panic!("Please provide a benchmark type as the first argument. Options: set-timeout, change-text, style, lot-element")
        });
        match benchmark_type.as_str() {
            "set-timeout" => {
                self.start_benchmark_set_timeout();
            }
            "change-text" => {
                self.start_benchmark_change_text();
            }
            "style" => {
                self.start_benchmark_styling();
            }
            "lot-element" => {
                let diff = self.start.elapsed();
                println!(
                    "Time to load {} elements: {}ns. Perfect score would be 0.",
                    self.benchmark_conig.iterations,
                    diff.as_nanos()
                );
                let start = std::time::Instant::now();
                self.qb
                    .b(Dom::get_elements_by_tag("Label").set_style("fg", "red"));
                self.process();
                let diff = start.elapsed();
                println!(
                    "Time to change style of {} elements: {}ns. Perfect score would be 0.",
                    self.benchmark_conig.iterations,
                    diff.as_nanos()
                );
            }
            _ => {}
        }
    }
}

fn create_window(_: WindowParams, _: &mut App<AppState>, _: WindowId) -> Windows {
    let mut window = BenchmarkMainWindow::new();
    window.post_construct();
    return Windows::BenchmarkMainWindow(window);
}

window_manager!(Windows {
        BenchmarkMainWindow,
    };
    create_window;
    AppState;
    WindowParams
);

fn main() -> AppResult {
    run_app(WindowParams {}, AppState {})
}
