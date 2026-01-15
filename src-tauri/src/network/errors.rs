use std::net::AddrParseError;
use thiserror::Error;
use crate::crypto::errors::CryptoError;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error(transparent)]
    CryptoError(#[from] CryptoError),

    #[error("Network Internal Error: {0}")]
    NetworkInternalError(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] tauri::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    AddrParseError(#[from] AddrParseError),
}

#[derive(serde::Serialize)]
#[serde(tag = "name", content = "message")]
#[serde(rename_all = "camelCase")]
enum NetworkErrorName {
    CryptoError(String),
    NetworkInternalError(String),
    Io(String),
    Other(String),
    Json(String),
    AddrParseError(String),
}

impl serde::Serialize for NetworkError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let message = self.to_string();

        let name = match self {
            Self::CryptoError(_) => NetworkErrorName::CryptoError(message),
            Self::NetworkInternalError(_) => {
                NetworkErrorName::NetworkInternalError(message)
            }
            Self::Io(_) => NetworkErrorName::Io(message),
            Self::Other(_) => NetworkErrorName::Other(message),
            Self::Json(_) => NetworkErrorName::Json(message),
            Self::AddrParseError(_) => {
                NetworkErrorName::AddrParseError(message)
            }
        };

        name.serialize(serializer)
    }
}
