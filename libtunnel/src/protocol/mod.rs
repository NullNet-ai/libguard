mod hash;
mod messages;

pub use hash::{str_hash, Hash};
pub use messages::{Message, Payload, PAYLOAD_SIZE};
use nullnet_liberror::{location, Error, ErrorHandler, Location};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn write_message(stream: &mut TcpStream, message: Message) -> Result<(), Error> {
    let data = message.serialize()?;

    stream.write_all(&data).await.handle_err(location!())?;
    stream.flush().await.handle_err(location!())?;

    Ok(())
}

pub async fn write_with_confirmation(stream: &mut TcpStream, message: Message) -> Result<(), Error> {
    write_message(stream, message).await?;

    // @TODO: Timeout needs to be implemented
    match expect_confirmation_message(stream).await {
        Ok(Message::Acknowledgment) => Ok(()),
        Ok(Message::Rejection) => Err("Received rejection").handle_err(location!()),
        Ok(_) => Err("Unexpected message").handle_err(location!()),
        Err(err) => Err(err),
    }
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
    let len_bytes = Message::len_bytes(&Message::DataConnectionRequest(Payload::default()));

    let message = expect_message(stream, len_bytes).await?;

    Ok(message)
}

pub async fn expect_confirmation_message(stream: &mut TcpStream) -> Result<Message, Error> {
    // We can get length either from Acknowledgment or from Rejection
    // Messages lengths are expected to be the same

    let len_bytes = Message::len_bytes(&Message::Acknowledgment);

    let message = expect_message(stream, len_bytes).await?;

    Ok(message)
}
