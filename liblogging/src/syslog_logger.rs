use syslog::{BasicLogger, Facility, Formatter3164};

pub enum SyslogServer {
    Local,
    Remote(String),
}

pub(crate) struct SyslogLogger {
    logger: BasicLogger,
}

impl SyslogLogger {
    pub fn new(syslog_server: SyslogServer, process_name: &str) -> Self {
        let formatter = Formatter3164 {
            facility: Facility::LOG_USER,
            hostname: None,
            process: process_name.to_string(),
            pid: std::process::id(),
        };

        let logger = BasicLogger::new(
            match syslog_server {
                SyslogServer::Local => syslog::unix(formatter),
                SyslogServer::Remote(server) => syslog::tcp(formatter, server),
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
