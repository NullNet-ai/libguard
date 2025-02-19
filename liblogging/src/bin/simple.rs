use nullnet_liblogging::Logger;

fn main() {
    Logger::init(None, "sample_app", vec!["simple"]);

    loop {
        log::error!("This is an error message");
        log::warn!("This is a warning message");
        log::info!("This is an info message");
        log::debug!("This is a debug message");
        log::trace!("This is a trace message");

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
