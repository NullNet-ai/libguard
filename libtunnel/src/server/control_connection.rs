use nullnet_liberror::{location, Error, ErrorHandler, Location};
use std::net::SocketAddr;
use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;

use super::profile::ClientProfile;
use crate::{protocol, Message};

pub struct ControlConnection {
    pub(crate) handle: JoinHandle<()>,
    pub(crate) visitor_rx: mpsc::Receiver<TcpStream>,
    pub(crate) shutdown_tx: broadcast::Sender<()>,
}

impl ControlConnection {
    pub fn new(stream: TcpStream, profile: &ClientProfile) -> Self {
        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);
        let (visitor_tx, visitor_rx) = mpsc::channel(1024);

        let addr = profile.visitor_addr;
        let handle =
            tokio::spawn(async move { Self::run(stream, addr, visitor_tx, shutdown_rx).await });

        Self {
            handle,
            visitor_rx,
            shutdown_tx,
        }
    }

    pub async fn shutdown(self) -> Result<(), Error> {
        let _ = self.shutdown_tx.send(()).handle_err(location!());
        self.handle.await.handle_err(location!())
    }

    pub async fn open_data_channel(&mut self, mut client_stream: TcpStream) -> Result<(), Error> {
        let visitor = self.visitor_rx.recv().await;

        if visitor.is_none() {
            return Err("Failed to receive a visitor").handle_err(location!());
        }

        let mut visitor_stream = visitor.unwrap();

        tokio::spawn(async move {
            println!("Data channel established");
            let _ = copy_bidirectional(&mut client_stream, &mut visitor_stream).await;
        });
        Ok(())
    }

    async fn run(
        stream: TcpStream,
        addr: SocketAddr,
        visitor_tx: mpsc::Sender<TcpStream>,
        mut shutdown_rx: broadcast::Receiver<()>,
    ) {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                println!("Control connection received a shutdown signal");
            },
            result = Self::accept_visitors(stream, addr, visitor_tx) => {
                if let Err(error) = result {
                    println!("Control connection error: {}", error.to_str())
                }
            }
        }

        println!("Control connection is terminated");
        // @TODO: Notify the manager
    }

    async fn accept_visitors(
        mut stream: TcpStream,
        addr: SocketAddr,
        visitor_tx: mpsc::Sender<TcpStream>,
    ) -> Result<(), Error> {
        let listener = TcpListener::bind(addr).await.handle_err(location!())?;

        loop {
            let (visitor, addr) = listener.accept().await.handle_err(location!())?;
            println!("Accepted visitor from: {}", addr);

            protocol::write_message(&mut stream, Message::ForwardConnectionRequest).await?;
            visitor_tx.send(visitor).await.handle_err(location!())?;
        }
    }
}
