use crate::crypto::{CipherInstance, CryptoRequest};
use crate::crypto::block::aes256::AES256;
use crate::crypto::block::BlockCipher;
use crate::crypto::block::modes::BlockMode;
use crate::crypto::block::modes::ofb::OfbMode;
use crate::crypto::block::xtea::XTea;
use crate::crypto::errors::CryptoError;
use crate::crypto::padding::Padding;
use crate::crypto::padding::pkcs7::Pkcs7;
use crate::crypto::stream::a5_1::A51Cipher;
use crate::crypto::stream::StreamCipher;

pub struct CipherFactory;

impl CipherFactory {
    pub fn create(req: CryptoRequest) -> Result<CipherInstance, CryptoError> {
        if req.algorithm.starts_with("stream:") {
            let cipher = match req.algorithm.as_str() {
                "stream:a5/1" => Box::new(A51Cipher::new(req.key)?) as Box<dyn StreamCipher>,
                _ => return Err(CryptoError::UnknownCipher(req.algorithm)),
            };
            return Ok(CipherInstance::Stream(cipher));
        }

        if !req.algorithm.starts_with("block:") {
            return Err(CryptoError::UnknownCipher(req.algorithm));
        }

        let cipher: Box<dyn BlockCipher> = match req.algorithm.as_str() {
            "block:xtea" => Box::new(XTea::new(req.key)?),
            "block:aes256" => Box::new(AES256::new(req.key)?),
            _ => return Err(CryptoError::UnknownCipher(req.algorithm)),
        };

        let mode_str = req.mode.ok_or(CryptoError::MissingBlockMode)?;
        let iv = req.iv.ok_or(CryptoError::MissingIV)?;

        let mode: Box<dyn BlockMode> = match mode_str.as_str() {
            "mode:ofb" => Box::new(OfbMode::new(iv)),
            _ => return Err(CryptoError::UnknownBlockMode(mode_str)),
        };

        let padding = match req.padding {
            None => None,
            Some(pad_str) => {
                let pad: Box<dyn Padding> = match pad_str.as_str() {
                    "pkcs7" => Box::new(Pkcs7),
                    _ => return Err(CryptoError::UnknownPadding(pad_str)),
                };
                Some(pad)
            }
        };

        if mode.needs_padding() && padding.is_none() {
            return Err(CryptoError::PaddingRequired);
        }

        Ok(CipherInstance::Block {
            cipher,
            mode,
            padding,
        })
    }
}
