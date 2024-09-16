use crate::error::TsunaguError;
use crate::Result;
use ring::{aead, rand::{self, SecureRandom}};

pub struct Encryption {
    key: aead::LessSafeKey,
    key_bytes: [u8; 32],
}

impl Encryption {
    /// Create a new Encryption instance with a randomly generated key
    pub fn new() -> Self {
        let rng = rand::SystemRandom::new();
        let mut key_bytes = [0u8; 32]; // AES-256 uses 32-byte keys
        rng.fill(&mut key_bytes).expect("Failed to generate random key");
        let key =
            aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)
                .unwrap();
        let key = aead::LessSafeKey::new(key);
        Self { key, key_bytes }
    }

    /// Encrypt the given data
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let nonce = aead::Nonce::assume_unique_for_key([0u8; 12]);
        let aad = aead::Aad::empty();

        let mut in_out = data.to_vec();
        self.key
            .seal_in_place_append_tag(nonce, aad, &mut in_out)
            .map_err(|e| TsunaguError::Encryption(e.to_string()))?;

        Ok(in_out)
    }

    /// Decrypt the given data
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>> {
        let nonce = aead::Nonce::assume_unique_for_key([0u8; 12]);
        let aad = aead::Aad::empty();

        let mut in_out = encrypted_data.to_vec();
        let decrypted_data = self
            .key
            .open_in_place(nonce, aad, &mut in_out)
            .map_err(|e| TsunaguError::Encryption(e.to_string()))?;

        Ok(decrypted_data.to_vec())
    }

    /// Get the base64 encoded key for sharing
    pub fn get_base64_key(&self) -> String {
        base64::encode(self.key_bytes)
    }

    /// Create an Encryption instance from a base64 encoded key
    pub fn from_base64_key(base64_key: &str) -> Result<Self> {
        let key_bytes = base64::decode(base64_key)
            .map_err(|e| TsunaguError::Encryption(format!("Invalid base64 key: {}", e)))?;

        let key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)
            .map_err(|e| TsunaguError::Encryption(format!("Invalid key: {}", e)))?;
        let key = aead::LessSafeKey::new(key);

        let mut key_array = [0u8; 32];
        key_array.copy_from_slice(&key_bytes);

        Ok(Self { key, key_bytes: key_array })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let encryption = Encryption::new();
        let original_data = b"Hello, World!";

        let encrypted = encryption.encrypt(original_data).unwrap();
        assert_ne!(encrypted, original_data);

        let decrypted = encryption.decrypt(&encrypted).unwrap();
        assert_eq!(decrypted, original_data);
    }

    #[test]
    fn test_base64_key() {
        let encryption = Encryption::new();
        let base64_key = encryption.get_base64_key();

        let encryption_from_key = Encryption::from_base64_key(&base64_key).unwrap();

        let original_data = b"Test data";
        let encrypted = encryption.encrypt(original_data).unwrap();
        let decrypted = encryption_from_key.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, original_data);
    }

    #[test]
    fn test_invalid_base64_key() {
        let result = Encryption::from_base64_key("invalid_base64");
        assert!(result.is_err());
    }
}
