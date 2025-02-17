mod console_logger;
mod error;
mod syslog_logger;

use crate::console_logger::ConsoleLogger;
use crate::syslog_logger::SyslogLogger;
pub use crate::syslog_logger::SyslogServer;
pub use error::{Error, ErrorHandler, Location};
use log::LevelFilter;
use std::str::FromStr;

pub struct Logger {
    syslog: SyslogLogger,
    console: ConsoleLogger,
}

impl Logger {
    pub fn init(syslog_server: SyslogServer, process_name: &str) {
        let env_log_level = std::env::var("LOG_LEVEL").unwrap_or("trace".to_string());
        let level_filter = LevelFilter::from_str(&env_log_level).unwrap_or(LevelFilter::Trace);
        if level_filter.to_level().is_some() {
            log::set_boxed_logger(Box::new(Logger {
                syslog: SyslogLogger::new(syslog_server, process_name),
                console: ConsoleLogger::new(),
            }))
            .unwrap_or_default();
        }
        log::set_max_level(level_filter);
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.syslog.enabled(metadata) || self.console.enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        self.syslog.log(record);
        self.console.log(record);
    }

    fn flush(&self) {
        self.syslog.flush();
        self.console.flush();
    }
}
