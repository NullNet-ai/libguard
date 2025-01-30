use base64::{engine::general_purpose, Engine as _};

/// Encodes binary data into a Base64-encoded string.
///
/// # Type Parameters
/// * `T` - Any type that implements `AsRef<[u8]>`, allowing flexible input types such as `Vec<u8>`, `&[u8]`, and `String`.
///
/// # Arguments
/// * `data` - The input data to be encoded. It can be any type that can be referenced as a byte slice (`AsRef<[u8]>`).
///
/// # Returns
/// A `String` containing the Base64-encoded representation of the input data.
pub fn encode_base64<T>(data: T) -> String
where
    T: AsRef<[u8]>,
{
    general_purpose::STANDARD.encode(data)
}
