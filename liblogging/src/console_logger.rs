use log::Log;

pub(crate) struct ConsoleLogger {}

impl ConsoleLogger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Log for ConsoleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let message = format!("[{}] {}", record.level(), record.args());
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
