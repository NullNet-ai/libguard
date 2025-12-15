#[tokio::main]
async fn main() {
    let logger_config = nullnet_liblogging::LoggerConfig::new(true, true, None, vec!["libtunnel"]);
    nullnet_liblogging::Logger::init(logger_config);
}
