use crate::crypto::errors::CryptoError;
use crate::crypto::stream::StreamCipher;

pub struct A51Cipher {
    x: u32, // 19 bits
    y: u32, // 22 bits
    z: u32, // 23 bits

    key: u64,
}

impl A51Cipher {
    pub fn new(key: Vec<u8>) -> Result<A51Cipher, CryptoError> {
        if key.len() != 8 {
            return Err(CryptoError::InvalidKeyLength);
        }
        let key_num = u64::from_le_bytes(key[0..8].try_into().unwrap());
        Ok(Self {
            x: (key_num >> 45) as u32,
            y: ((key_num >> 23) & 0x3fffff) as u32,
            z: (key_num & 0x7ffff) as u32,
            key: key_num,
        })
    }

    fn maj(&self) -> u32 {
        let x8 = (self.x >> 8) & 1;
        let y10 = (self.y >> 10) & 1;
        let z10 = (self.z >> 10) & 1;
        if x8 + y10 + z10 > 1 { 1 } else { 0 }
    }

    fn shift_x(&mut self) {
        let x13 = self.x >> 13;
        let x16 = self.x >> 16;
        let x17 = self.x >> 17;
        let x18 = self.x >> 18;
        self.x <<= 1;
        self.x |= (x13 ^ x16 ^ x17 ^ x18) & 1;
    }

    fn shift_y(&mut self) {
        let y20 = self.y >> 20;
        let y21 = self.y >> 21;
        self.y <<= 1;
        self.y |= (y20 ^ y21) & 1;
    }

    fn shift_z(&mut self) {
        let z7 = self.z >> 7;
        let z20 = self.z >> 20;
        let z21 = self.z >> 21;
        let z22 = self.z >> 22;
        self.z <<= 1;
        self.z |= (z7 ^ z20 ^ z21 ^ z22) & 1;
    }

    fn next_bit(&mut self) -> u8 {
        let m = self.maj();
        if (self.x >> 8) & 1 == m {
            self.shift_x();
        }
        if (self.y >> 10) & 1 == m {
            self.shift_y();
        }
        if (self.z >> 10) & 1 == m {
            self.shift_z();
        }
        (((self.x >> 18) ^ (self.y >> 21) ^ (self.z >> 22)) & 1) as u8
    }

    fn next_byte(&mut self) -> u8 {
        let mut res = 0u8;
        for _ in 0..8 {
            res <<= 1;
            res |= self.next_bit()
        }
        res
    }
}

impl StreamCipher for A51Cipher {
    fn encrypt_byte(&mut self, byte: u8) -> Result<u8, CryptoError> {
        let n = self.next_byte();
        Ok(byte ^ n)
    }

    fn decrypt_byte(&mut self, byte: u8) -> Result<u8, CryptoError> {
        Ok(byte ^ self.next_byte())
    }

    fn reset(&mut self) -> Result<(), CryptoError> {
        self.x = (self.key >> 45) as u32;
        self.y = ((self.key >> 23) & 0x3fffff) as u32;
        self.z = (self.key & 0x7ffff) as u32;
        Ok(())
    }
}