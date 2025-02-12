use nullnet_liblogging::{location, ErrorHandler, Location, Logger, SyslogServer};

fn main() {
    Logger::init(SyslogServer::Remote("127.0.0.1:514".to_string()));

    loop {
        Logger::info("This is an info message from client");
        Logger::warn("This is a warning message from client".to_string());
        Logger::debug("This is a debug message from client");
        Logger::trace("This is a trace message from client".to_string());
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
