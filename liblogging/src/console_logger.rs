use log::Log;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const RESET: &str = "\x1b[0m";

pub(crate) struct ConsoleLogger {
    enabled: bool,
}

impl ConsoleLogger {
    pub(crate) fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Log for ConsoleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.enabled && metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let message = format_message(record);
            match record.level() {
                log::Level::Error | log::Level::Warn => {
                    eprintln!("{message}");
                }
                _ => println!("{message}"),
            };
        }
    }

    fn flush(&self) {}
}

fn format_message(record: &log::Record) -> String {
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, true);
    let level = record.level();
    let message = record.args();
    let color = match level {
        log::Level::Error => RED,
        log::Level::Warn => YELLOW,
        log::Level::Info => GREEN,
        log::Level::Debug => BLUE,
        log::Level::Trace => MAGENTA,
    };
    format!("{now} {color}[{level}]{RESET} {message}")
}
