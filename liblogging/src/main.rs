// Sample program to test the logging library

use nullnet_liblogging::{DatastoreCredentials, Logger, LoggerConfig};

#[tokio::main]
async fn main() {
    // read first command line argument
    let args: Vec<String> = std::env::args().collect();
    let default_runner = "sample program".to_string();
    let runner = args.get(1).unwrap_or(&default_runner);

    let logger_config = LoggerConfig::new(
        true,
        false,
        Some(DatastoreCredentials::new("app_id", "app_secret")),
        Vec::new(),
    );

    Logger::init(logger_config);

    loop {
        log::error!("This is an error message from {runner}");
        log::warn!("This is a warning message from {runner}");
        log::info!("This is an info message from {runner}");
        log::debug!("This is a debug message from {runner}");
        log::trace!("This is a trace message from {runner}");

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
