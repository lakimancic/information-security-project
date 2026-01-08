use serde::Deserialize;
use crate::crypto::block::BlockCipher;
use crate::crypto::block::modes::BlockMode;
use crate::crypto::stream::StreamCipher;

mod stream;
mod errors;
mod block;
mod cipher_factory;

pub enum CipherInstance {
    Stream(Box<dyn StreamCipher>),
    Block {
        cipher: Box<dyn BlockCipher>,
        mode: Box<dyn BlockMode>
    }
}

#[derive(Debug, Deserialize)]
pub struct CryptoRequest {
    pub algorithm: String,
    pub mode: Option<String>,
    pub key: Vec<u8>,
    pub iv: Option<Vec<u8>>,
}