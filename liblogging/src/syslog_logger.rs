use syslog::{BasicLogger, Facility, Formatter3164};

/// Possible syslog endpoints
pub enum SyslogEndpoint {
    /// Use the local syslog server
    Local,
    /// Use a remote syslog server
    Remote(String),
}

pub(crate) struct SyslogLogger {
    logger: Option<BasicLogger>,
}

impl SyslogLogger {
    pub(crate) fn new(syslog_endpoint: Option<SyslogEndpoint>, process_name: &str) -> Self {
        if let Some(syslog_endpoint) = syslog_endpoint {
            let formatter = Formatter3164 {
                facility: Facility::LOG_USER,
                hostname: None,
                process: process_name.to_string(),
                pid: std::process::id(),
            };

            let logger = BasicLogger::new(
                match syslog_endpoint {
                    SyslogEndpoint::Local => syslog::unix(formatter),
                    SyslogEndpoint::Remote(server) => syslog::tcp(formatter, server),
                }
                .expect("could not connect to syslog server"),
            );

            Self {
                logger: Some(logger),
            }
        } else {
            Self { logger: None }
        }
    }
}

impl log::Log for SyslogLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.logger
            .as_ref()
            .is_some_and(|logger| logger.enabled(metadata))
    }

    fn log(&self, record: &log::Record) {
        if let Some(logger) = self.logger.as_ref() {
            logger.log(record);
        }
    }

    fn flush(&self) {
        if let Some(logger) = self.logger.as_ref() {
            logger.flush();
        }
    }
}
