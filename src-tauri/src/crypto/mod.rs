use serde::Deserialize;
use crate::crypto::block::BlockCipher;
use crate::crypto::block::modes::BlockMode;
use crate::crypto::padding::Padding;
use crate::crypto::stream::StreamCipher;

mod stream;
pub(crate) mod errors;
pub(crate) mod block;
mod cipher_factory;
pub(crate) mod hash;
pub(crate) mod encryptor;
pub(crate) mod padding;
pub mod api;
pub mod commands;

pub enum CipherInstance {
    Stream(Box<dyn StreamCipher>),
    Block {
        cipher: Box<dyn BlockCipher>,
        mode: Box<dyn BlockMode>,
        padding: Box< dyn Padding>,
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CryptoRequest {
    pub algorithm: String,
    pub padding: Option<String>,
    pub mode: Option<String>,
    pub key: Vec<u8>,
    pub iv: Option<Vec<u8>>,
}