// file containing simple logger implementation

use chrono::Utc;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub enum LogLevel {
    ERROR,
    WARNING,
    INFORMATIONAL,
}

pub struct MyLogger {
    fd: File,
    log_level: LogLevel,
}

impl MyLogger {
    pub fn log(&mut self, record: &str) {
        let date_str = Utc::now().format("%Y-%m-%d %H:%M:%S.%f").to_string();
        write!(self.fd, "{} > {} - {}\n", date_str, "INFORMATIONAL", record);
    }
}

pub fn create_logger(filename: &str) -> MyLogger {
    MyLogger {
        //fd: File::open(filename).unwrap(),
        fd: OpenOptions::new().create(true).append(true).open(filename).unwrap(),
        log_level: LogLevel::INFORMATIONAL,
    }
}
