use syslog::{BasicLogger, Facility, Formatter3164};

#[derive(Default)]
pub(crate) struct SyslogLogger {
    logger: Option<BasicLogger>,
}

impl SyslogLogger {
    pub(crate) fn new(syslog_endpoint: bool) -> Self {
        if !syslog_endpoint {
            return Self::default();
        }

        let formatter = Formatter3164 {
            facility: Facility::LOG_USER,
            hostname: None,
            process: String::from("nullnet"),
            pid: std::process::id(),
        };

        let logger = BasicLogger::new(
            syslog::unix(formatter).expect("could not connect to the local syslog socket"),
        );

        Self {
            logger: Some(logger),
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
