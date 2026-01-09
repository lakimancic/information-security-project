use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Unknown cipher: {0}")]
    UnknownCipher(String),

    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    #[error("Invalid key length")]
    InvalidKeyLength,

    #[error("Invalid IV length")]
    InvalidIvLength,

    #[error("Encryption error: {0}")]
    EncryptionError(String),

    #[error("Invalid block size")]
    InvalidBlockSize,

    #[error("Missing block mode")]
    MissingBlockMode,

    #[error("Invalid padding")]
    InvalidPadding,

    #[error("Missing padding algorithm")]
    MissingPaddingAlgorithm,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}