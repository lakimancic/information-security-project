pub mod blake_256;

use crate::crypto::errors::CryptoError;

pub trait HashFunction {
    fn digest_size(&self) -> usize;
    fn update(&mut self, data: &[u8]) -> Result<(), CryptoError>;
    fn finalize(&mut self) -> Result<Vec<u8>, CryptoError>;
    fn reset(&mut self) -> Result<(), CryptoError>;
}