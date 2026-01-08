use crate::crypto::block::BlockCipher;
use crate::crypto::block::modes::BlockMode;
use crate::crypto::errors::CryptoError;

pub struct OfbMode {
    iv: Vec<u8>,
    state: Vec<u8>,
}

impl OfbMode {
    pub fn new(iv: Vec<u8>) -> Self { Self { state: iv.clone(), iv: iv.clone() } }
}

impl BlockMode for OfbMode {
    fn encrypt_next(&mut self, cipher: &dyn BlockCipher, block: &mut [u8]) -> Result<(), CryptoError> {
        cipher.encrypt_block(&mut self.state)?;
        for (b, k) in block.iter_mut().zip(&self.state) {
            *b ^= k;
        }
        Ok(())
    }

    fn decrypt_next(&mut self, cipher: &dyn BlockCipher, block: &mut [u8]) -> Result<(), CryptoError> {
        cipher.encrypt_block(self.state.as_mut_slice())?;
        for (b, k) in block.iter_mut().zip(&self.state) {
            *b ^= k;
        }
        Ok(())
    }

    fn reset(&mut self) -> Result<(), CryptoError> {
        self.state = self.iv.clone();
        Ok(())
    }
}