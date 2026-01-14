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