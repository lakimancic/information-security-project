pub mod a5_1;

use crate::crypto::errors::CryptoError;

pub trait StreamCipher {
    fn encrypt_byte(&mut self, byte: u8) -> Result<u8, CryptoError>;
    fn decrypt_byte(&mut self, byte: u8) -> Result<u8, CryptoError>;

    fn reset(&mut self) -> Result<(), CryptoError>;
}