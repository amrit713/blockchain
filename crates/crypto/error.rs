use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum CryptoError {
    #[error("Invalid public key format: expected 32 bytes")]
    InvalidPublicKeyBytes,

    #[error("Invalid signature format: expected 64 bytes")]
    InvalidSignatureBytes,

    #[error("Signature verification failed: invalid signature or tampered data")]
    VerificationFailed,

    #[error("Failed to generate secure random keypair: {0}")]
    KeyGenerationFailed(String),

    #[error("Hex decoding failure: {0}")]
    HexDecodeError(String),
}
