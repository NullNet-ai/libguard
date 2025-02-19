use syslog::{BasicLogger, Facility, Formatter3164};

/// Possible syslog endpoints
pub enum SyslogEndpoint {
    /// Use the local syslog server
    Local,
    /// Use a remote syslog server
    Remote(String),
}

pub(crate) struct SyslogLogger {
    logger: BasicLogger,
}

impl SyslogLogger {
    pub(crate) fn new(syslog_endpoint: SyslogEndpoint, process_name: &str) -> Self {
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

        Self { logger }
    }
}

impl log::Log for SyslogLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.logger.enabled(metadata)
    }

    fn log(&self, record: &log::Record) {
        self.logger.log(record);
    }

    fn flush(&self) {
        self.logger.flush();
    }
}
