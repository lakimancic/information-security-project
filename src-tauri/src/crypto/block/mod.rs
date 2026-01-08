pub mod modes;
pub mod xtea;
mod aes256;

use crate::crypto::errors::CryptoError;

pub trait BlockCipher {
    fn block_size(&self) -> usize;

    fn encrypt_block(&self, block: &mut [u8]) -> Result<(), CryptoError>;
    fn decrypt_block(&self, block: &mut [u8]) -> Result<(), CryptoError>;
}