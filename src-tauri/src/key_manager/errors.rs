use thiserror::Error;
use crate::crypto::errors::CryptoError;

#[derive(Error, Debug)]
pub enum KeysError {
    #[error("Failed to generate new key (and IV)")]
    GenerateKeyError,

    #[error("Key with name '{0}' already exists")]
    KeyAlreadyExists(String),

    #[error("Key with name '{0}' not found")]
    KeyNotFound(String),

    #[error("Invalid key password")]
    InvalidPassword,

    #[error(transparent)]
    CryptoError(#[from] CryptoError),

    #[error("Invalid format")]
    InvalidFormat,

    #[error("Invalid key name")]
    InvalidName,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}