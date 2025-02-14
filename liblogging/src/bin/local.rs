use nullnet_liblogging::{location, ErrorHandler, Location, Logger, SyslogServer};

fn main() {
    Logger::init(SyslogServer::Local, log::LevelFilter::Info, "sample_app");

    loop {
        log::info!("This is an info message");
        log::warn!("This is a warning message");
        log::debug!("This is a debug message");
        log::trace!("This is a trace message");
        let _ = fallible_method().handle_err(location!());

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

// this models any external method that can fail
// it returns an error kind that depends on the external library implementation
// it will be logged and transformed in our `Error` type by the error handler
fn fallible_method() -> std::io::Result<Vec<u8>> {
    std::fs::read("non_existent_file.txt")
}
