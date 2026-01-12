use thiserror::Error;

#[derive(Debug, thiserror::Error)]
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

    #[error("Crypto Internal Error: {0}")]
    CryptoInternalError(String),

    #[error("File is not encrypting")]
    FileIsNotEncrypting,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] tauri::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

#[derive(serde::Serialize)]
#[serde(tag = "name", content = "message")]
#[serde(rename_all = "camelCase")]
enum ErrorName {
    UnknownCipher(String),
    InvalidParams(String),
    InvalidKeyLength(String),
    InvalidIvLength(String),
    EncryptionError(String),
    InvalidBlockSize(String),
    MissingBlockMode(String),
    InvalidPadding(String),
    MissingPaddingAlgorithm(String),
    CryptoInternalError(String),
    FileIsNotEncrypting(String),
    Io(String),
    Other(String),
    Json(String),
}

impl serde::Serialize for CryptoError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let message = self.to_string();
        let name = match self {
            Self::UnknownCipher(_) => ErrorName::UnknownCipher(message),
            Self::InvalidParams(_) => ErrorName::InvalidParams(message),
            Self::InvalidKeyLength => ErrorName::InvalidKeyLength(message),
            Self::InvalidIvLength => ErrorName::InvalidIvLength(message),
            Self::EncryptionError(_) => ErrorName::EncryptionError(message),
            Self::InvalidBlockSize => ErrorName::InvalidBlockSize(message),
            Self::MissingBlockMode => ErrorName::MissingBlockMode(message),
            Self::InvalidPadding => ErrorName::InvalidPadding(message),
            Self::MissingPaddingAlgorithm => ErrorName::MissingPaddingAlgorithm(message),
            Self::CryptoInternalError(_) => ErrorName::CryptoInternalError(message),
            Self::FileIsNotEncrypting => ErrorName::FileIsNotEncrypting(message),
            Self::Io(_) => ErrorName::Io(message),
            Self::Other(_) => ErrorName::Other(message),
            Self::Json(_) => ErrorName::Json(message),
        };
        name.serialize(serializer)
    }
}