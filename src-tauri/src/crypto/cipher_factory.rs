use crate::crypto::{CipherInstance, CryptoRequest};
use crate::crypto::block::modes::ofb::OfbMode;
use crate::crypto::block::xtea::XTea;
use crate::crypto::errors::CryptoError;
use crate::crypto::stream::a5_1::A51Cipher;

pub struct CipherFactory;

impl CipherFactory {
    pub fn create(req: CryptoRequest) -> Result<CipherInstance, CryptoError> {
        let mode = match req.mode {
            None => None,
            Some(mode_str) => if let Some(iv_str) = req.iv {
                match mode_str.as_str() {
                    "ofb" => Some(Box::new(OfbMode::new(iv_str))),
                    _ => None
                }
            } else { None }
        };

        match req.algorithm.as_str() {
            "a5/1" => {
                let cipher = Box::new(A51Cipher::new(req.key)?);
                Ok(CipherInstance::Stream(cipher))
            }
            "xtea" => {
                let cipher = Box::new(XTea::new(req.key)?);
                if let Some(mode_box) = mode {
                    Ok(CipherInstance::Block {
                        cipher,
                        mode: mode_box
                    })
                }
                else {
                    Err(CryptoError::MissingBlockMode)
                }
            }
            _ => Err(CryptoError::UnknownCipher(req.algorithm)),
        }
    }
}