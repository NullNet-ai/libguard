mod config;
pub use config::*;

use crate::{protocol, str_hash, Hash, Message, Payload};
use nullnet_liberror::{location, Error, ErrorHandler, Location};
use tokio::{io::copy_bidirectional, net::TcpStream};

pub struct Client {
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&self) {
        // @TODO: Implement reconnection in case of an error
        let _ = Self::run_control_connectinon(self.config.clone()).await;
    }

    async fn run_control_connectinon(config: Config) -> Result<(), Error> {
        println!("Requesting control connection from the server");

        let mut server_stream = TcpStream::connect(&config.server_addr)
            .await
            .handle_err(location!())?;

        let hash = str_hash(&config.id);
        Self::request_open_control_connection(&mut server_stream, hash.clone()).await?;

        println!("Control connection established");

        loop {
            match Self::await_for_forwarding_request(&mut server_stream).await {
                Err(err) => {
                    println!("Error happened when wating for 'ForwardConnectionRequest' message. {err:?}");
                    Err(err)?;
                }
                _ => {
                    let config = config.clone();
                    tokio::spawn(async move {
                        println!("Received ForwardConnectionRequest message");
                        match Self::run_data_connection(config).await {
                            Ok(_) => println!("Data connection terminated"),
                            Err(err) => println!("Data connection error: {err:?}"),
                        }
                    });
                }
            };
        }

    }

    async fn run_data_connection(config: Config) -> Result<(), Error> {
        println!("Requesting data connection from the server");

        let mut server_stream = TcpStream::connect(&config.server_addr)
            .await
            .handle_err(location!())?;

        let hash = str_hash(&config.id);
        Self::request_open_data_connection(&mut server_stream, hash.clone()).await?;

        let mut local_stream = TcpStream::connect(&config.local_addr)
            .await
            .handle_err(location!())?;

        copy_bidirectional(&mut server_stream, &mut local_stream)
            .await
            .handle_err(location!())?;

        Ok(())
    }

    async fn request_open_control_connection(
        stream: &mut TcpStream,
        hash: Hash,
    ) -> Result<(), Error> {
        let open_message = Message::ControlConnectionRequest(Payload { data: hash });
        protocol::write_with_confirmation(stream, open_message).await
    }

    async fn request_open_data_connection(stream: &mut TcpStream, hash: Hash) -> Result<(), Error> {
        let open_message = Message::DataConnectionRequest(Payload { data: hash });
        protocol::write_with_confirmation(stream, open_message).await
    }

    async fn await_for_forwarding_request(stream: &mut TcpStream) -> Result<(), Error> {
        let message_size = Message::len_bytes(&Message::ForwardConnectionRequest);
        match protocol::expect_message(stream, message_size).await {
            Ok(Message::ForwardConnectionRequest) => Ok(()),
            Ok(_) => Err("Unexpected message").handle_err(location!()),
            Err(err) => Err(err),
        }
    }
}
