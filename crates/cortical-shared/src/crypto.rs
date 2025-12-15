//! Cryptographic services for inter-brain communication.
//!
//! Implements X25519 key exchange and ChaCha20Poly1305 encryption
//! based on omni-core patterns.

use base64::Engine;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use x25519_dalek::{PublicKey, StaticSecret};

/// Brain keypair for X25519 key exchange
#[derive(Clone)]
pub struct BrainKeyPair {
    secret: StaticSecret,
    public: PublicKey,
}

impl BrainKeyPair {
    /// Generate a new random keypair
    pub fn generate() -> Self {
        let secret = StaticSecret::random_from_rng(rand::thread_rng());
        let public = PublicKey::from(&secret);
        Self { secret, public }
    }

    /// Restore from hex-encoded secret key
    pub fn from_secret_hex(secret_hex: &str) -> Result<Self, CryptoError> {
        let bytes = hex::decode(secret_hex).map_err(|_| CryptoError::InvalidSecretKey)?;
        let arr: [u8; 32] = bytes.try_into().map_err(|_| CryptoError::InvalidSecretKey)?;
        let secret = StaticSecret::from(arr);
        let public = PublicKey::from(&secret);
        Ok(Self { secret, public })
    }

    /// Get public key as bytes
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.public.to_bytes()
    }

    /// Get public key as hex string
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.public.to_bytes())
    }

    /// Get secret key as hex string (for secure storage)
    pub fn secret_key_hex(&self) -> String {
        hex::encode(self.secret.as_bytes())
    }

    /// Derive shared secret from another brain's public key
    pub fn derive_shared_secret(&self, other_public: &[u8; 32]) -> [u8; 32] {
        let other_public = PublicKey::from(*other_public);
        self.secret.diffie_hellman(&other_public).to_bytes()
    }

    /// Derive shared secret from hex-encoded public key
    pub fn derive_shared_secret_hex(&self, other_public_hex: &str) -> Result<[u8; 32], CryptoError> {
        let bytes = parse_public_key(other_public_hex)?;
        Ok(self.derive_shared_secret(&bytes))
    }
}

/// Encrypted message envelope for inter-brain communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedEnvelope {
    /// Sender brain ID (plaintext for routing)
    pub from_brain: String,
    /// Recipient brain ID (plaintext for routing)
    pub to_brain: String,
    /// Base64-encoded nonce (12 bytes)
    pub nonce: String,
    /// Base64-encoded ciphertext
    pub ciphertext: String,
}

impl EncryptedEnvelope {
    /// Encrypt a message using the shared secret
    pub fn encrypt(
        from_brain: &str,
        to_brain: &str,
        plaintext: &[u8],
        shared_secret: &[u8; 32],
    ) -> Result<Self, CryptoError> {
        let cipher = ChaCha20Poly1305::new_from_slice(shared_secret)
            .map_err(|_| CryptoError::InvalidKey)?;

        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|_| CryptoError::EncryptionFailed)?;

        let b64 = base64::engine::general_purpose::STANDARD;
        Ok(Self {
            from_brain: from_brain.to_string(),
            to_brain: to_brain.to_string(),
            nonce: b64.encode(nonce_bytes),
            ciphertext: b64.encode(ciphertext),
        })
    }

    /// Decrypt the message using the shared secret
    pub fn decrypt(&self, shared_secret: &[u8; 32]) -> Result<Vec<u8>, CryptoError> {
        let b64 = base64::engine::general_purpose::STANDARD;

        let nonce_bytes: [u8; 12] = b64
            .decode(&self.nonce)
            .map_err(|_| CryptoError::InvalidNonce)?
            .try_into()
            .map_err(|_| CryptoError::InvalidNonce)?;

        let ciphertext = b64
            .decode(&self.ciphertext)
            .map_err(|_| CryptoError::InvalidCiphertext)?;

        let cipher = ChaCha20Poly1305::new_from_slice(shared_secret)
            .map_err(|_| CryptoError::InvalidKey)?;

        let nonce = Nonce::from_slice(&nonce_bytes);

        cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|_| CryptoError::DecryptionFailed)
    }
}

/// Cryptographic errors
#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Invalid key")]
    InvalidKey,
    #[error("Invalid secret key")]
    InvalidSecretKey,
    #[error("Invalid public key")]
    InvalidPublicKey,
    #[error("Invalid nonce")]
    InvalidNonce,
    #[error("Invalid ciphertext")]
    InvalidCiphertext,
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
}

/// Parse hex-encoded public key
pub fn parse_public_key(hex_key: &str) -> Result<[u8; 32], CryptoError> {
    let bytes = hex::decode(hex_key).map_err(|_| CryptoError::InvalidPublicKey)?;
    bytes.try_into().map_err(|_| CryptoError::InvalidPublicKey)
}

/// Generate a random challenge for handshake
pub fn generate_challenge() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Compute HMAC-SHA256 for challenge response
pub fn compute_challenge_response(challenge: &str, shared_secret: &[u8; 32]) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let challenge_bytes = hex::decode(challenge).unwrap_or_default();
    let mut mac: HmacSha256 = Mac::new_from_slice(shared_secret).expect("HMAC accepts any key size");
    mac.update(&challenge_bytes);
    hex::encode(mac.finalize().into_bytes())
}

/// Verify challenge response
pub fn verify_challenge_response(
    challenge: &str,
    response: &str,
    shared_secret: &[u8; 32],
) -> bool {
    let expected = compute_challenge_response(challenge, shared_secret);
    // Constant-time comparison
    expected == response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_exchange_and_encryption() {
        // Brain A generates keypair
        let brain_a = BrainKeyPair::generate();
        let a_public = brain_a.public_key_bytes();

        // Brain B generates keypair
        let brain_b = BrainKeyPair::generate();
        let b_public = brain_b.public_key_bytes();

        // Both derive the same shared secret
        let a_shared = brain_a.derive_shared_secret(&b_public);
        let b_shared = brain_b.derive_shared_secret(&a_public);
        assert_eq!(a_shared, b_shared);

        // Encrypt with A's shared secret
        let plaintext = b"Hello from Brain A!";
        let envelope = EncryptedEnvelope::encrypt(
            "brain-a",
            "brain-b",
            plaintext,
            &a_shared,
        ).unwrap();

        // Decrypt with B's shared secret (same as A's)
        let decrypted = envelope.decrypt(&b_shared).unwrap();
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_keypair_serialization() {
        let original = BrainKeyPair::generate();
        let secret_hex = original.secret_key_hex();
        let public_hex = original.public_key_hex();

        let restored = BrainKeyPair::from_secret_hex(&secret_hex).unwrap();
        assert_eq!(restored.public_key_hex(), public_hex);
    }

    #[test]
    fn test_challenge_response() {
        let keypair_a = BrainKeyPair::generate();
        let keypair_b = BrainKeyPair::generate();
        
        let shared = keypair_a.derive_shared_secret(&keypair_b.public_key_bytes());
        let challenge = generate_challenge();
        let response = compute_challenge_response(&challenge, &shared);
        
        assert!(verify_challenge_response(&challenge, &response, &shared));
        assert!(!verify_challenge_response(&challenge, "wrong", &shared));
    }
}
