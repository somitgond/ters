// file containing simple logger implementation

use chrono::Utc;
use std::fs::{File, OpenOptions};
use std::io::Write;

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel {
    INFORMATIONAL,
    WARNING,
    ERROR,
}

pub struct MyLogger {
    fd: File,
    log_level: LogLevel,
}

impl MyLogger {
    pub fn get_log_level_str(log_lvl: LogLevel) -> String {
        match log_lvl {
            LogLevel::ERROR => String::from("ERR"),
            LogLevel::WARNING => String::from("WARN"),
            _ => String::from("INFO"),
        }
    }

    pub fn log(&mut self, record: &str, log_lvl: LogLevel) {
        if log_lvl >= self.log_level {
            let date_str = Utc::now().format("%Y-%m-%d %H:%M:%S.%f").to_string();

            let _ = write!(
                self.fd,
                "{} > {} - {}\n",
                date_str,
                Self::get_log_level_str(log_lvl),
                record
            );
        }
    }
}

pub fn create_logger(filename: &str, log_lvl: LogLevel) -> MyLogger {
    MyLogger {
        fd: OpenOptions::new()
            .create(true)
            .append(true)
            .open(filename)
            .unwrap(),
        log_level: log_lvl,
    }
}
