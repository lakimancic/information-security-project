use crate::crypto::{CipherInstance, CryptoRequest};
use crate::crypto::block::aes256::AES256;
use crate::crypto::block::BlockCipher;
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
        let mode = match req.mode {
            None => None,
            Some(mode_str) => if let Some(iv_str) = req.iv {
                match mode_str.as_str() {
                    "mode:ofb" => Some(Box::new(OfbMode::new(iv_str))),
                    _ => None
                }
            } else { None }
        };

        let padding = match req.padding {
            None => None,
            Some(pad_str) =>
                match pad_str.as_str() {
                    "pkcs7" => Some(Box::new(Pkcs7) as Box<dyn Padding>),
                    _ => None
                }
        };

        if req.algorithm.starts_with("stream:") {
            let cipher = match req.algorithm.as_str() {
                "stream:a5/1" => {
                    Box::new(A51Cipher::new(req.key)?) as Box<dyn StreamCipher>
                }
                _ => return Err(CryptoError::UnknownCipher(req.algorithm))
            };
            Ok(CipherInstance::Stream(cipher))
        }
        else if req.algorithm.starts_with("block:") {
            let cipher: Box<dyn BlockCipher> = match req.algorithm.as_str() {
                "block:xtea" => {
                    Box::new(XTea::new(req.key)?) as Box<dyn BlockCipher>
                }
                "block:aes256" => {
                    Box::new(AES256::new(req.key)?) as Box<dyn BlockCipher>
                }
                _ => return Err(CryptoError::UnknownCipher(req.algorithm)),
            };
            if let Some(mode_box) = mode {
                if let Some(padding_box) = padding {
                    Ok(CipherInstance::Block {
                        cipher,
                        mode: mode_box,
                        padding: padding_box
                    })
                }
                else {
                    Err(CryptoError::MissingPaddingAlgorithm)
                }
            }
            else {
                Err(CryptoError::MissingBlockMode)
            }
        }
        else {
            Err(CryptoError::UnknownCipher(req.algorithm))
        }
    }
}