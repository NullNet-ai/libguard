#![doc = include_str!("../README.md")]

mod console_logger;
mod error;
mod syslog_logger;

use crate::console_logger::ConsoleLogger;
pub use crate::syslog_logger::SyslogEndpoint;
use crate::syslog_logger::SyslogLogger;
pub use error::{Error, ErrorHandler, Location};
use log::LevelFilter;
use std::str::FromStr;

static DEFAULT_ALLOWED_TARGETS: once_cell::sync::Lazy<Vec<&'static str>> =
    once_cell::sync::Lazy::new(|| vec!["nullnet", "appguard", "wallguard"]);

/// Logger implementation that logs to both syslog and console
pub struct Logger {
    syslog: SyslogLogger,
    console: ConsoleLogger,
    allowed_targets: Vec<&'static str>,
}

impl Logger {
    /// Initializes the logger with the given syslog server, process name, and allowed targets
    ///
    /// # Arguments
    /// * `syslog_server` - The syslog server to log to
    /// * `process_name` - The name of the process
    /// * `allowed_targets` - The list of allowed targets; if any are specified,
    ///   only logs from targets starting with one of these entries will be printed.
    pub fn init(
        syslog_server: SyslogEndpoint,
        process_name: &str,
        allowed_targets: Vec<&'static str>,
    ) {
        let env_log_level = std::env::var("LOG_LEVEL").unwrap_or("trace".to_string());
        let level_filter = LevelFilter::from_str(&env_log_level).unwrap_or(LevelFilter::Trace);
        if level_filter.to_level().is_some() {
            log::set_boxed_logger(Box::new(Logger {
                syslog: SyslogLogger::new(syslog_server, process_name),
                console: ConsoleLogger::new(),
                allowed_targets,
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
        if DEFAULT_ALLOWED_TARGETS.contains(&record.target())
            || self.allowed_targets.contains(&record.target())
        {
            self.syslog.log(record);
            self.console.log(record);
        }
    }

    fn flush(&self) {
        self.syslog.flush();
        self.console.flush();
    }
}
