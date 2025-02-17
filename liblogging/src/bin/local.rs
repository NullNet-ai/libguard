use nullnet_liblogging::{location, ErrorHandler, Location, Logger, SyslogEndpoint};

fn main() {
    Logger::init(SyslogEndpoint::Local, "sample_app");

    loop {
        let _ = fallible_method().handle_err(location!());
        log::warn!("This is a warning message");
        log::info!("This is an info message");
        log::debug!("This is a debug message");
        log::trace!("This is a trace message");

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

// this models any external method that can fail
// it returns an error kind that depends on the external library implementation
// it will be logged and transformed in our `Error` type by the error handler
fn fallible_method() -> std::io::Result<Vec<u8>> {
    std::fs::read("non_existent_file.txt")
}
