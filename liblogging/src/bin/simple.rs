use nullnet_liblogging::{Logger, PostgresEndpoint};

fn main() {
    let postgres_endpoint = PostgresEndpoint::new(
        "localhost".to_string(),
        5432,
        "admin".to_string(),
        "admin".to_string(),
        "logs".to_string(),
        "log_messages".to_string(),
    );
    Logger::init(None, Some(postgres_endpoint), "sample_app", vec!["simple"]);

    loop {
        log::error!("This is an error message");
        log::warn!("This is a warning message");
        log::info!("This is an info message");
        log::debug!("This is a debug message");
        log::trace!("This is a trace message");

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
