use crate::key_manager::errors::KeysError;
use crate::crypto::errors::CryptoError;

pub struct KeySizes {}

impl KeySizes {
    pub fn get_size_of(algorithm: &String, mode: &Option<String>) -> Result<(usize, usize), KeysError> {
        let (key_size, block_size) : (usize, usize) = match algorithm.as_str() {
            "stream:a5/1" => Ok((8, 0)),
            "block:xtea" => Ok((16, 8)),
            "block:aes256" => Ok((32, 32)),
            _ => Err(CryptoError::UnknownCipher(algorithm.clone()))
        }?;

        let iv_size = match mode {
            Some(mode_str) => match mode_str.as_str() {
                "mode:ofb" => block_size,
                _ => 0
            },
            None => 0
        };

        Ok((key_size, iv_size))
    }
}