#![doc = include_str!("../README.md")]

use std::iter::{IntoIterator, Iterator};
use std::str::FromStr;

use log::LevelFilter;

use crate::console_logger::ConsoleLogger;
use crate::postgres_logger::PostgresLogger;
use crate::syslog_logger::SyslogLogger;

mod console_logger;
mod postgres_logger;
mod syslog_logger;

static DEFAULT_ALLOWED_TARGETS: once_cell::sync::Lazy<Vec<String>> =
    once_cell::sync::Lazy::new(|| {
        vec!["nullnet", "appguard", "wallguard"]
            .into_iter()
            .map(str::to_lowercase)
            .collect()
    });

/// Logger implementation that logs to both syslog and console
pub struct Logger {
    syslog: SyslogLogger,
    console: ConsoleLogger,
    postgres: PostgresLogger,
    allowed_targets: Vec<String>,
}

impl Logger {
    /// Initializes the logger with the given syslog server, process name, and allowed targets
    ///
    /// # Arguments
    /// * `syslog_endpoint` - The syslog endpoint to log to
    /// * `process_name` - The name of the process
    /// * `allowed_targets` - The list of allowed targets; if any are specified,
    ///   only logs from targets starting with one of these entries will be printed.
    pub fn init(
        syslog_endpoint: bool,
        console_logger: bool,
        postgres_endpoint: bool,
        allowed_targets: Vec<&'static str>,
    ) {
        let env_log_level = std::env::var("LOG_LEVEL").unwrap_or("trace".to_string());
        let level_filter = LevelFilter::from_str(&env_log_level).unwrap_or(LevelFilter::Trace);
        if level_filter.to_level().is_some() {
            let allowed_targets = allowed_targets.into_iter().map(str::to_lowercase).collect();
            log::set_boxed_logger(Box::new(Logger {
                syslog: SyslogLogger::new(syslog_endpoint),
                console: ConsoleLogger::new(console_logger),
                postgres: PostgresLogger::new(postgres_endpoint),
                allowed_targets,
            }))
            .unwrap_or_default();
        }
        log::set_max_level(level_filter);
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.syslog.enabled(metadata)
            || self.console.enabled(metadata)
            || self.postgres.enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        let target = record.target().to_lowercase();
        if DEFAULT_ALLOWED_TARGETS
            .iter()
            .any(|s| target.starts_with(s))
            || self.allowed_targets.iter().any(|s| target.starts_with(s))
        {
            self.syslog.log(record);
            self.console.log(record);
            self.postgres.log(record);
        }
    }

    fn flush(&self) {
        self.syslog.flush();
        self.console.flush();
        self.postgres.flush();
    }
}
