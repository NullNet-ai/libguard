#![doc = include_str!("../README.md")]

use std::iter::{IntoIterator, Iterator};
use std::str::FromStr;

use log::LevelFilter;

use crate::console_logger::ConsoleLogger;
use crate::postgres_logger::PostgresLogger;
use crate::syslog_logger::SyslogLogger;

mod console_logger;
mod datastore;
mod postgres_logger;
mod syslog_logger;

static DEFAULT_ALLOWED_TARGETS: once_cell::sync::Lazy<Vec<String>> =
    once_cell::sync::Lazy::new(|| {
        vec!["nullnet", "appguard", "wallguard"]
            .into_iter()
            .map(str::to_lowercase)
            .collect()
    });

/// Logger implementation that logs to console, syslog, and `PostgreSQL`
pub struct Logger {
    console: ConsoleLogger,
    syslog: SyslogLogger,
    postgres: PostgresLogger,
    allowed_targets: Vec<String>,
}

impl Logger {
    /// Initializes the logger with the given configuration
    ///
    /// # Arguments
    /// * `logger_config` - The logger configuration
    pub fn init(logger_config: LoggerConfig) {
        let LoggerConfig {
            console,
            syslog,
            postgres,
            allowed_targets,
        } = logger_config;

        let env_log_level = std::env::var("LOG_LEVEL").unwrap_or("trace".to_string());
        let level_filter = LevelFilter::from_str(&env_log_level).unwrap_or(LevelFilter::Trace);
        if level_filter.to_level().is_some() {
            let allowed_targets = allowed_targets.into_iter().map(str::to_lowercase).collect();
            log::set_boxed_logger(Box::new(Logger {
                console: ConsoleLogger::new(console),
                syslog: SyslogLogger::new(syslog),
                postgres: PostgresLogger::new(postgres),
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

/// Logger configuration
pub struct LoggerConfig {
    /// Whether to log to console
    pub console: bool,
    /// Whether to log to syslog
    pub syslog: bool,
    /// Whether to log to `PostgreSQL`
    pub postgres: bool,
    /// The list of allowed targets.<br>
    ///   By default, only logs from `nullnet*`, `appguard*`, and `wallguard*` will be emitted.<br>
    ///   Use this parameter to specify additional targets
    ///   (e.g., specifying "serde" will emit logs for all targets whose name is in the form `serde*`).
    pub allowed_targets: Vec<&'static str>,
}

impl LoggerConfig {
    /// Creates a new logger configuration
    ///
    /// # Arguments
    /// * `console_logger` - Whether to log to console
    /// * `syslog_endpoint` - Whether to log to syslog
    /// * `postgres_endpoint` - Whether to log to `PostgreSQL`
    /// * `allowed_targets` - The list of allowed targets.<br>
    ///   By default, only logs from `nullnet*`, `appguard*`, and `wallguard*` will be emitted.<br>
    ///   Use this parameter to specify additional targets
    ///   (e.g., specifying "serde" will emit logs for all targets whose name is in the form `serde*`).
    #[must_use]
    pub fn new(
        console: bool,
        syslog: bool,
        postgres: bool,
        allowed_targets: Vec<&'static str>,
    ) -> Self {
        Self {
            console,
            syslog,
            postgres,
            allowed_targets,
        }
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            console: true,
            syslog: true,
            postgres: true,
            allowed_targets: vec![],
        }
    }
}
