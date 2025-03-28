use clap::Parser;
use nullnet_libtunnel::{Profile, ServerConfig};
use std::net::SocketAddr;

#[derive(Parser, Debug, Clone)]
struct Args {
    #[arg(long, default_value = "client")]
    pub mode: String,

    #[arg(long, default_value = "127.0.0.1:9000")]
    pub server_addr: String,

    #[arg(long, default_value = "127.0.0.1:5000")]
    pub service_addr: String,
}

struct ClientProfile {}

impl Profile for ClientProfile {
    fn get_unique_id(&self) -> String {
        String::from("test")
    }

    fn get_visitor_addr(&self) -> SocketAddr {
        "0.0.0.0:8080".parse().unwrap()
    }

    fn get_visitor_token(&self) -> Option<String> {
        None
    }
}

#[tokio::main]
async fn main() {
    let logger_config = nullnet_liblogging::LoggerConfig::new(true, true, None, vec!["libtunnel"]);
    nullnet_liblogging::Logger::init(logger_config);

    let args = Args::parse();

    if args.mode.to_lowercase() == "client" {
        let server_addr = args.server_addr.parse().expect("Wrond server bind addr");
        let local_addr = args.service_addr.parse().expect("Wrong local addr");

        let config = nullnet_libtunnel::ClientConfig {
            id: String::from("test"),
            server_addr,
            local_addr,
            reconnect_timeout: None,
        };

        let client = nullnet_libtunnel::Client::new(config);

        let _ = tokio::signal::ctrl_c().await;

        client.shutdown().await;
    } else if args.mode.to_lowercase() == "server" {
        let profile = ClientProfile {};
        let config = ServerConfig::default();
        let server = nullnet_libtunnel::Server::new(config);

        server
            .insert_profile(profile)
            .await
            .expect("Failed to register client profile");

        let _ = tokio::signal::ctrl_c().await;

        server.shutdown().await;
    } else {
        panic!("Unsupported mode: {}", args.mode);
    }
}
