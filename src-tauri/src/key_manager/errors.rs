use thiserror::Error;
use crate::crypto::errors::CryptoError;

#[derive(Error, Debug)]
pub enum KeysError {
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

    #[error("Key Manager Internal Error: {0}")]
    KeyManagerInternalError(String),
}

#[derive(serde::Serialize)]
#[serde(tag = "name", content = "message")]
#[serde(rename_all = "camelCase")]
enum KeysErrorName {
    KeyAlreadyExists(String),
    KeyNotFound(String),
    InvalidPassword(String),
    CryptoError(String),
    InvalidFormat(String),
    InvalidName(String),
    Io(String),
    KeyManagerInternalError(String),
}

impl serde::Serialize for KeysError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let message = self.to_string();

        let name = match self {
            Self::KeyAlreadyExists(_) => KeysErrorName::KeyAlreadyExists(message),
            Self::KeyNotFound(_) => KeysErrorName::KeyNotFound(message),
            Self::InvalidPassword => KeysErrorName::InvalidPassword(message),
            Self::CryptoError(_) => KeysErrorName::CryptoError(message),
            Self::InvalidFormat => KeysErrorName::InvalidFormat(message),
            Self::InvalidName => KeysErrorName::InvalidName(message),
            Self::Io(_) => KeysErrorName::Io(message),
            Self::KeyManagerInternalError(_) => {
                KeysErrorName::KeyManagerInternalError(message)
            }
        };

        name.serialize(serializer)
    }
}