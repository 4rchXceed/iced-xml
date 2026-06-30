const LOG_FILE: &str = "log.txt";

pub fn init() {
    let log_file = std::fs::File::create(LOG_FILE);
    if log_file.is_err() {
        panic!("Failed to create log file: {}", LOG_FILE);
    }
}

pub fn log(message: &str) {
    println!("{}", message);
}

pub fn fatal(message: &str) {
    eprintln!("{}", message);
    panic!("Fatal error, see logs");
}
