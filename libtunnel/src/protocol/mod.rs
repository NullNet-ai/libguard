mod messages;

pub use messages::{Message, Payload};

use nullnet_liberror::{location, Error, ErrorHandler, Location};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn write_message(stream: &mut TcpStream, message: Message) -> Result<(), Error> {
    let data = message.serialize()?;

    stream.write_all(&data).await.handle_err(location!())?;
    stream.flush().await.handle_err(location!())?;

    Ok(())
}

pub async fn expect_message(stream: &mut TcpStream, expected_len: usize) -> Result<Message, Error> {
    let mut buffer = vec![0; expected_len];

    stream
        .read_exact(&mut buffer)
        .await
        .handle_err(location!())?;

    let message = Message::deserialize(&buffer)?;
    Ok(message)
}

pub async fn expect_open_message(stream: &mut TcpStream) -> Result<Message, Error> {
    // We can get length either from DataConnectionRequest or from ControlConnectionRequest
    // Messages lengths are expected to be the same
    let len_bytes = Message::DataConnectionRequest(Payload::default()).len_bytes();

    let message = expect_message(stream, len_bytes).await?;

    Ok(message)
}

