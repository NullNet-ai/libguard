use crate::{protocol, Hash, Message, Payload};
use nullnet_liberror::{location, Error, ErrorHandler, Location};
use tokio::net::TcpStream;

/// Sends an open session request to the server over the given TCP stream.
///
/// This function prepares a `Message::OpenSessionRequest` message with the provided hash
/// and sends it to the server, waiting for confirmation before returning.
///
/// # Parameters
/// - `stream`: A mutable reference to the `TcpStream` used to communicate with the server.
/// - `hash`: A hash representing a unique identifier used in the session request.
///
/// # Returns
/// - `Ok(())`: If the session open request was successfully sent and confirmed.
/// - `Err(Error)`: If there was an error in sending the message or receiving confirmation.
pub(super) async fn request_open_session(stream: &mut TcpStream, hash: Hash) -> Result<(), Error> {
    let open_message = Message::OpenSessionRequest(Payload { data: hash });
    protocol::write_with_confirmation(stream, open_message).await
}

/// Sends an open channel request to the server over the given TCP stream.
///
/// This function prepares a `Message::OpenChannelRequest` message with the provided hash
/// and sends it to the server, waiting for confirmation before returning.
///
/// # Parameters
/// - `stream`: A mutable reference to the `TcpStream` used to communicate with the server.
/// - `hash`: A hash representing a unique identifier used in the channel request.
///
/// # Returns
/// - `Ok(())`: If the channel open request was successfully sent and confirmed.
/// - `Err(Error)`: If there was an error in sending the message or receiving confirmation.
pub(super) async fn request_open_channel(stream: &mut TcpStream, hash: Hash) -> Result<(), Error> {
    let open_message = Message::OpenChannelRequest(Payload { data: hash });
    protocol::write_with_confirmation(stream, open_message).await
}

/// Waits for a channel request message from the server.
///
/// This function waits for an incoming message of type `Message::ForwardConnectionRequest`.
/// If the received message matches the expected type, it is returned; otherwise, an error is raised.
///
/// # Parameters
/// - `stream`: A mutable reference to the `TcpStream` used to receive messages from the server.
///
/// # Returns
/// - `Ok(Message::ForwardConnectionRequest)`: If the expected message is received.
/// - `Err(Error)`: If an unexpected message is received or if there is an error in receiving the message.
pub(super) async fn await_channel_request(stream: &mut TcpStream) -> Result<Message, Error> {
    let message_size = Message::len_bytes(&Message::ForwardConnectionRequest);
    let message = protocol::expect_message(stream, message_size).await?;
    match message {
        Message::ForwardConnectionRequest => Ok(message),
        _ => Err("Unexpected message").handle_err(location!()),
    }
}
