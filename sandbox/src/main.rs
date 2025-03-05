use clap::Parser;
use nullnet_liblogging::LoggerConfig;
// use nullnet_liblogging::LoggerConfig;
use std::{net::SocketAddr, time::Duration};

#[derive(Parser, Debug, Clone)]
struct Args {
    #[arg(long, default_value = "client")]
    pub mode: String,

    #[arg(long, default_value = "127.0.0.1:9000")]
    pub server_addr: String,

    #[arg(long, default_value = "127.0.0.1:8080")]
    pub visitor_addr: String,

    #[arg(long, default_value = "127.0.0.1:5000")]
    pub service_addr: String,
}

#[tokio::main]
async fn main() {
    // nullnet_liblogging::Logger::init(LoggerConfig::default());

    nullnet_liblogging::Logger::init(LoggerConfig::new(true, false, None, vec!["sandbox, nullnet"]));

    let args = Args::parse();

    if args.mode.to_lowercase() == "client" {
        let server_addr = args.server_addr.parse().expect("Wrond server bind addr");
        let local_addr = args.service_addr.parse().expect("Wrong local addr");

        let config = libtunnel::Config {
            id: String::from("test"),
            server_addr,
            local_addr,
            heartbeat_timeout: Some(Duration::from_secs(5 * 60)),
            reconnect_timeout: Some(Duration::from_secs(60)),
        };

        let mut client = libtunnel::Client::new(config);

        let _ = client.run().await;
    } else if args.mode.to_lowercase() == "server" {
        let visitor_addr: SocketAddr = args.visitor_addr.parse().expect("Wrong visitor address");

        let profile = libtunnel::ClientProfile {
            id: String::from("test"),
            token: String::from("not_used_yet"),
            visitor_addr: visitor_addr,
        };

        let server_addr = args.server_addr.parse().expect("Wrond server bind addr");

        let mut server = libtunnel::Server::new(server_addr, Some(Duration::from_secs(15)));

        server
            .register_profile(profile)
            .await
            .expect("Failed to register client profile");

        let _ = server.run().await;
    } else {
        panic!("Unsupported mode: {}", args.mode);
    }
}
