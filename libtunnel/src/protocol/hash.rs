use super::PAYLOAD_SIZE;
use sha2::{Digest, Sha256};

pub type Hash = [u8; PAYLOAD_SIZE];

pub fn str_hash(value: &str) -> Hash {
    Sha256::digest(value.as_bytes()).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_str_hash_length() {
        let hash = str_hash("Hello, World!");
        assert_eq!(hash.len(), PAYLOAD_SIZE);
    }

    #[test]
    fn test_str_hash_consistency() {
        let input = "consistent input";
        let hash1 = str_hash(input);
        let hash2 = str_hash(input);
        assert_eq!(
            hash1, hash2,
            "Hash output should be consistent for the same input"
        );
    }

    #[test]
    fn test_str_hash_different_inputs() {
        let hash1 = str_hash("input one");
        let hash2 = str_hash("input two");
        assert_ne!(
            hash1, hash2,
            "Different inputs should produce different hashes"
        );
    }

    #[test]
    fn test_str_hash_empty_string() {
        let hash = str_hash("");
        let expected: [u8; 32] = Sha256::digest(b"").into();
        assert_eq!(
            hash, expected,
            "Hash of empty string should match SHA-256 output"
        );
    }
}
