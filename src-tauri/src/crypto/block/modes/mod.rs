pub mod ofb;

use crate::crypto::block::BlockCipher;
use crate::crypto::errors::CryptoError;

pub trait BlockMode {
    fn encrypt_next(&mut self, cipher: &dyn BlockCipher, block: &mut [u8]) -> Result<(), CryptoError>;
    fn decrypt_next(&mut self, cipher: &dyn BlockCipher, block: &mut [u8]) -> Result<(), CryptoError>;

    fn needs_padding(&self) -> bool;

    fn reset(&mut self) -> Result<(), CryptoError>;
}