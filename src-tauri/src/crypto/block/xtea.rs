use crate::crypto::block::BlockCipher;
use crate::crypto::errors::CryptoError;

const NUM_ROUNDS: usize = 32;
const DELTA: u32 = 0x9E3779B9;
const BLOCK_SIZE: usize = 8;

pub struct XTea {
    key: [u32; 4],
}

impl XTea {
    pub fn new(key: Vec<u8>) -> Result<Self, CryptoError> {
        if key.len() != 16 {
            return Err(CryptoError::InvalidKeyLength);
        }
        let chunks = key.chunks_exact(4);
        let words: [u32; 4] = chunks
            .map(|c| u32::from_be_bytes(c.try_into().unwrap()))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Ok(Self { key: words })
    }
}

impl BlockCipher for XTea {
    fn block_size(&self) -> usize {
        BLOCK_SIZE
    }

    fn encrypt_block(&self, block: &mut [u8]) -> Result<(), CryptoError> {
        if block.len() != BLOCK_SIZE {
            return Err(CryptoError::InvalidBlockSize);
        }
        let mut sum : u32 = 0;
        let mut v0 = u32::from_be_bytes(block[0..4].try_into().unwrap());
        let mut v1 = u32::from_be_bytes(block[4..8].try_into().unwrap());
        for _ in 0..NUM_ROUNDS {
            v0 = v0.wrapping_add(v1.wrapping_add((v1 << 4) ^ (v1 >> 5)) ^ sum.wrapping_add(self.key[(sum & 3) as usize]));
            sum = sum.wrapping_add(DELTA);
            v1 = v1.wrapping_add(v0.wrapping_add((v0 << 4) ^ (v0 >> 5)) ^ sum.wrapping_add(self.key[((sum >> 11) & 3) as usize]));
        }
        block[0..4].copy_from_slice(&v0.to_be_bytes());
        block[4..8].copy_from_slice(&v1.to_be_bytes());
        Ok(())
    }

    fn decrypt_block(&self, block: &mut [u8]) -> Result<(), CryptoError> {
        if block.len() != BLOCK_SIZE {
            return Err(CryptoError::InvalidBlockSize);
        }
        let mut sum : u32 = DELTA.wrapping_mul(NUM_ROUNDS as u32);
        let mut v0 = u32::from_be_bytes(block[0..4].try_into().unwrap());
        let mut v1 = u32::from_be_bytes(block[4..8].try_into().unwrap());
        for _ in 0..NUM_ROUNDS {
            v1 = v1.wrapping_sub(v0.wrapping_add((v0 << 4) ^ (v0 >> 5)) ^ sum.wrapping_add(self.key[((sum >> 11) & 3) as usize]));
            sum = sum.wrapping_add(DELTA);
            v0 = v0.wrapping_sub(v1.wrapping_add((v1 << 4) ^ (v1 >> 5)) ^ sum.wrapping_add(self.key[(sum & 3) as usize]));
        }
        block[0..4].copy_from_slice(&v0.to_be_bytes());
        block[4..8].copy_from_slice(&v1.to_be_bytes());
        Ok(())
    }
}