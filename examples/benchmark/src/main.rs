use std::process::exit;

#[cfg(feature = "dev-mode")]
use iced_xml::app_wrapper::CssRx;
#[cfg(feature = "dev-mode")]
use iced_xml::app_wrapper::CssWatcher;
#[cfg(feature = "dev-mode")]
use iced_xml::utils::watch_css::watch_css_file;

use iced_xml::{
    app_wrapper::{AppResult, AppTemplate, Objects, ObjectsReadOnly, run_app},
    dom::{api::Dom, query::QueryBuilder},
    xml_engine::XmlEngine,
};

struct BenchmarkConfig {
    iterations: i32,
}

struct BenchmarkApp {
    qb: QueryBuilder<BenchmarkApp>,
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

impl BenchmarkApp {
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
        self.qb.set_interval(1).with_callback(|this, _| {
            this.i += 1;
            if this.i >= this.benchmark_conig.iterations {
                println!("Benchmark finished after {} iterations", this.i);
                let elapsed = this.start.elapsed();
                println!(
                    "Duration of {}ns. Perfect score would be 1'000'000'000.",
                    elapsed.as_nanos()
                );
                this.start = std::time::Instant::now();
                this.qb.set_interval(0).with_callback(|this, _| {
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
        self.qb.set_interval(0).with_callback(|this, _| {
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
}

impl AppTemplate<BenchmarkApp> for BenchmarkApp {
    fn new() -> Self {
        Self {
            qb: QueryBuilder::new(),
            engine: XmlEngine::new(include_bytes!("res/main.xml").to_vec()),
            #[cfg(feature = "dev-mode")]
            css_rx: None,
            #[cfg(feature = "dev-mode")]
            _keep_watcher: None,
            i: 0,
            benchmark_conig: BenchmarkConfig {
                iterations: 1000_000,
            },
            start: std::time::Instant::now(),
        }
    }

    fn get_objects(&mut self) -> Objects<'_, Self> {
        return Objects {
            engine: &mut self.engine,
            qb: &mut self.qb,
            #[cfg(feature = "dev-mode")]
            css_watcher_rx: self.css_rx.as_mut(),
            #[cfg(feature = "dev-mode")]
            css_path: "src/res/style.css",
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
        #[cfg(feature = "dev-mode")]
        self.watch_css();
        self.qb
            .b(Dom::get_element_by_id("start-set-timeout").add_event_listener("click"))
            .with_callback(|this, _| {
                this.start_benchmark_set_timeout();
            });
        self.qb
            .b(Dom::get_element_by_id("start-change-text").add_event_listener("click"))
            .with_callback(|this, _| {
                this.start_benchmark_change_text();
            });
        self.process();
    }

    #[cfg(feature = "dev-mode")]
    fn set_css_watcher_rx(&mut self, rx: CssRx) {
        self.css_rx = Some(rx);
    }
}

fn main() -> AppResult {
    return run_app::<BenchmarkApp>();
}
