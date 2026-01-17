use crate::crypto::errors::CryptoError;
use crate::crypto::padding::Padding;

pub struct Pkcs7;

impl Padding for Pkcs7 {
    fn pad(&self, data: &mut Vec<u8>, block_size: usize) -> Result<(), CryptoError> {
        let pad_len = block_size - (data.len() % block_size);
        data.extend(std::iter::repeat(pad_len as u8).take(pad_len));
        Ok(())
    }

    fn unpad(&self, data: &mut Vec<u8>, block_size: usize) -> Result<(), CryptoError> {
        if data.is_empty() || data.len() % block_size != 0 {
            return Err(CryptoError::InvalidPadding);
        }

        let pad_len = *data.last().unwrap() as usize;
        if pad_len == 0 || pad_len > block_size {
            return Err(CryptoError::InvalidPadding);
        }

        if !data[data.len() - pad_len..].iter().all(|&b| b as usize == pad_len) {
            return Err(CryptoError::InvalidPadding);
        }

        data.truncate(data.len() - pad_len);
        Ok(())
    }

    fn pad_size(&self, size: usize, block_size: usize) -> usize {
        block_size - size % block_size
    }
}
