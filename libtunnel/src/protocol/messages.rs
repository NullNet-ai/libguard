use nullnet_liberror::{Error, ErrorHandler, Location, location};
use serde::{Deserialize, Serialize};

pub const PAYLOAD_SIZE: usize = 32;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Payload {
    pub(crate) data: [u8; PAYLOAD_SIZE],
}

impl From<[u8; PAYLOAD_SIZE]> for Payload {
    fn from(value: [u8; PAYLOAD_SIZE]) -> Self {
        Self { data: value }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Message {
    OpenSessionRequest(Payload),
    OpenChannelRequest(Payload),
    Authenticate(Payload),
    ForwardConnectionRequest,
    Heartbeat,
    Acknowledgment,
    Rejection,
}

impl Message {
    pub fn len_bytes(&self) -> usize {
        bincode::serialize(self)
            .map(|bytes| bytes.len())
            .unwrap_or(0)
    }

    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        bincode::serialize(self).handle_err(location!())
    }

    pub fn deserialize(data: &[u8]) -> Result<Message, Error> {
        bincode::deserialize::<Message>(data).handle_err(location!())
    }
}

#[cfg(test)]
mod tests {
    use crate::str_hash;

    use super::*;

    #[test]
    fn test_message_serialization() {
        let msg = Message::ForwardConnectionRequest;
        let serialized = msg.serialize().expect("Failed to serialize message");
        assert!(
            !serialized.is_empty(),
            "Serialized data should not be empty"
        );
    }

    #[test]
    fn test_message_deserialization() {
        let msg = Message::ForwardConnectionRequest;
        let serialized = msg.serialize().expect("Failed to serialize message");
        let deserialized =
            Message::deserialize(&serialized).expect("Failed to deserialize message");

        assert_eq!(
            std::mem::discriminant(&msg),
            std::mem::discriminant(&deserialized),
            "Deserialized message should match original"
        );
    }

    #[test]
    fn test_control_connection_request_serialization() {
        let payload = Payload {
            data: [1; PAYLOAD_SIZE],
        };
        let msg = Message::OpenSessionRequest(payload);
        let serialized = msg.serialize().expect("Failed to serialize message");
        assert!(
            !serialized.is_empty(),
            "Serialized data should not be empty"
        );

        let deserialized =
            Message::deserialize(&serialized).expect("Failed to deserialize message");

        match deserialized {
            Message::OpenSessionRequest(p) => {
                assert_eq!(p.data, [1; PAYLOAD_SIZE], "Payload data mismatch")
            }
            _ => panic!("Deserialized message does not match original"),
        }
    }

    #[test]
    fn test_invalid_deserialization() {
        let invalid_data = vec![0, 1, 2, 3, 4]; // Random bytes, not a valid `Message`
        let result = Message::deserialize(&invalid_data);
        assert!(
            result.is_err(),
            "Deserialization should fail on invalid data"
        );
    }

    #[test]
    fn test_message_length() {
        let msg = Message::ForwardConnectionRequest;
        let length = msg.len_bytes();
        assert!(length > 0, "Message length should be greater than 0");
    }

    #[test]
    fn messages_are_distinguishable() {
        let s1 = Message::ForwardConnectionRequest.serialize().unwrap();
        let s2 = Message::Acknowledgment.serialize().unwrap();

        let payload = Payload {
            data: [1; PAYLOAD_SIZE],
        };
        let s3 = Message::OpenSessionRequest(payload.clone())
            .serialize()
            .unwrap();
        let s4 = Message::OpenChannelRequest(payload.clone())
            .serialize()
            .unwrap();

        assert!(s1 != s2);
        assert!(s2 != s3);
        assert!(s3 != s4);
        assert!(s1 != s4);
    }

    #[test]
    fn some_messages_must_have_the_same_length() {
        let payload = Payload::default();

        let m1 = Message::OpenSessionRequest(payload.clone())
            .serialize()
            .unwrap();
        let m2 = Message::OpenChannelRequest(payload.clone())
            .serialize()
            .unwrap();

        assert_eq!(m1.len(), m2.len());

        let m1 = Message::Acknowledgment.serialize().unwrap();
        let m2 = Message::Rejection.serialize().unwrap();
        assert_eq!(m1.len(), m2.len());

        let m1 = Message::ForwardConnectionRequest.serialize().unwrap();
        let m2 = Message::Heartbeat.serialize().unwrap();
        assert_eq!(m1.len(), m2.len());
    }

    #[test]

    fn can_transfer_hash() {
        let identifier = String::from("test");

        let hash = str_hash(&identifier);

        let msg = Message::OpenChannelRequest(Payload { data: hash.clone() });

        let msg = Message::deserialize(&msg.serialize().unwrap()).unwrap();

        match msg {
            Message::OpenChannelRequest(payload) => assert_eq!(payload.data, hash),
            _ => panic!(),
        }
    }
}
