use nullnet_liblogging::{Logger, SyslogEndpoint};

fn main() {
    Logger::init(
        SyslogEndpoint::Remote("127.0.0.1:514".to_string()),
        "sample_app",
        vec!["client"],
    );

    loop {
        log::error!("This is an error message");
        log::warn!("This is a warning message");
        log::info!("This is an info message");
        log::debug!("This is a debug message");
        log::trace!("This is a trace message");

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
