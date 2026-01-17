pub mod pkcs7;

use crate::crypto::errors::CryptoError;

pub trait Padding {
    fn pad(&self, data: &mut Vec<u8>, block_size: usize) -> Result<(), CryptoError>;
    fn unpad(&self, data: &mut Vec<u8>, block_size: usize) -> Result<(), CryptoError>;
    fn pad_size(&self, size: usize, block_size: usize) -> usize;
}