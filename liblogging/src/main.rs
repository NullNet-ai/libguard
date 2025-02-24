// <https://www.atlassian.com/data/sql/how-to-start-a-postgresql-server-on-mac-os-x>
// -------------------------------------------------------------------------------------------------
// To make it work remotely:
// 1. edit postgresql.conf and set listen_addresses = '*'
// 2. edit pg_hba.conf and add: "host all all 0.0.0.0/0	trust"
// -------------------------------------------------------------------------------------------------

use nullnet_liblogging::{Logger, LoggerConfig};

fn main() {
    // read first command line argument
    let args: Vec<String> = std::env::args().collect();
    let default_runner = "sample program".to_string();
    let runner = args.get(1).unwrap_or(&default_runner);

    Logger::init(LoggerConfig::default());

    loop {
        log::error!("This is an error message from {runner}");
        log::warn!("This is a warning message from {runner}");
        log::info!("This is an info message from {runner}");
        log::debug!("This is a debug message from {runner}");
        log::trace!("This is a trace message from {runner}");

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
