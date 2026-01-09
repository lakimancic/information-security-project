use crate::crypto::errors::CryptoError;
use crate::crypto::hash::HashFunction;

const IV: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const C: [u32; 16] = [
    0x243f6a88, 0x85a308d3, 0x13198a2e, 0x03707344,
    0xa4093822, 0x299f31d0, 0x082efa98, 0xec4e6c89,
    0x452821e6, 0x38d01377, 0xbe5466cf, 0x34e90c6c,
    0xc0ac29b7, 0xc97c50dd, 0x3f84d5b5, 0xb5470917,
];

const SIGMA: [[usize; 16]; 14] = [
    [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15 ],
    [14,10, 4, 8, 9,15,13, 6, 1,12, 0, 2,11, 7, 5, 3 ],
    [11, 8,12, 0, 5, 2,15,13,10,14, 3, 6, 7, 1, 9, 4 ],
    [ 7, 9, 3, 1,13,12,11,14, 2, 6, 5,10, 4, 0,15, 8 ],
    [ 9, 0, 5, 7, 2, 4,10,15,14, 1,11,12, 6, 8, 3,13 ],
    [ 2,12, 6,10, 0,11, 8, 3, 4,13, 7, 5,15,14, 1, 9 ],
    [12, 5, 1,15,14,13, 4,10, 0, 7, 6, 3, 9, 2, 8,11 ],
    [13,11, 7,14,12, 1, 3, 9, 5, 0,15, 4, 8, 6, 2,10 ],
    [ 6,15,14, 9,11, 3, 0, 8,12, 2,13, 7, 1, 4,10, 5 ],
    [10, 2, 8, 4, 7, 6, 1, 5,15,11, 9,14, 3,12,13, 0 ],
    [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15 ],
    [14,10, 4, 8, 9,15,13, 6, 1,12, 0, 2,11, 7, 5, 3 ],
    [11, 8,12, 0, 5, 2,15,13,10,14, 3, 6, 7, 1, 9, 4 ],
    [ 7, 9, 3, 1,13,12,11,14, 2, 6, 5,10, 4, 0,15, 8 ],
];

fn g(v: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize, r: usize, i: usize, m: &[u32; 16]) {
    v[a] = v[a].wrapping_add(v[b]).wrapping_add(m[SIGMA[r][i]] ^ C[SIGMA[r][i+1]]);
    v[d] = (v[d] ^ v[a]).rotate_right(16);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(12);

    v[a] = v[a].wrapping_add(v[b]).wrapping_add(m[SIGMA[r][i+1]] ^ C[SIGMA[r][i]]);
    v[d] = (v[d] ^ v[a]).rotate_right(8);
    v[c] = v[c].wrapping_add(v[d]);
    v[b] = (v[b] ^ v[c]).rotate_right(7);
}

pub struct Blake256 {
    h: [u32; 8],
    buffer: [u8; 64],
    buffer_len: usize,
    bit_len: u64,
}

impl Blake256 {
    pub fn new() -> Self {
        Self {
            h: IV,
            buffer: [0u8; 64],
            buffer_len: 0,
            bit_len: 0,
        }
    }

    fn compress(&mut self, block: &[u8; 64]) {
        let mut m = [0u32; 16];
        for i in 0..16 {
            m[i] = u32::from_be_bytes(block[i*4..i*4+4].try_into().unwrap());
        }

        let mut v = [0u32; 16];
        v[0..8].copy_from_slice(&self.h);
        v[8..12].copy_from_slice(&[0; 4]);
        v[8..12].iter_mut().enumerate().for_each(|(i, val)| *val ^= C[i]);
        v[12..16].copy_from_slice(&C[4..8]);

        let t_low = self.bit_len as u32;
        let t_high = (self.bit_len >> 32) as u32;
        v[12] ^= t_low;
        v[13] ^= t_low;
        v[14] ^= t_high;
        v[15] ^= t_high;

        for r in 0..14 {
            g(&mut v, 0, 4, 8, 12, r, 0, &m);
            g(&mut v, 1, 5, 9, 13, r, 2, &m);
            g(&mut v, 2, 6, 10, 14, r, 4, &m);
            g(&mut v, 3, 7, 11, 15, r, 6, &m);
            g(&mut v, 0, 5, 10, 15, r, 8, &m);
            g(&mut v, 1, 6, 11, 12, r, 10, &m);
            g(&mut v, 2, 7, 8, 13, r, 12, &m);
            g(&mut v, 3, 4, 9, 14, r, 14, &m);
        }

        for i in 0..8 {
            self.h[i] ^= v[i] ^ v[i + 8];
        }
    }

}

impl HashFunction for Blake256 {
    fn digest_size(&self) -> usize {
        32
    }

    fn update(&mut self, data: &[u8]) -> Result<(), CryptoError> {
        for &b in data {
            self.buffer[self.buffer_len] = b;
            self.buffer_len += 1;

            if self.buffer_len == 64 {
                self.bit_len += 512;
                let block = self.buffer;
                self.compress(&block);
                self.buffer_len = 0;
            }
        }
        Ok(())
    }

    fn finalize(&mut self) -> Result<Vec<u8>, CryptoError> {
        let mut out = vec![0u8; 32];

        self.bit_len += (self.buffer_len as u64) * 8;

        self.buffer[self.buffer_len] = 0x80;
        self.buffer_len += 1;

        if self.buffer_len > 55 {
            for i in self.buffer_len..64 {
                self.buffer[i] = 0;
            }
            let block = self.buffer;
            self.compress(&block);
            self.buffer = [0u8; 64];
        }

        for i in self.buffer_len..55 {
            self.buffer[i] = 0;
        }

        self.buffer[55] = 0x01;

        self.buffer[56..64].copy_from_slice(&self.bit_len.to_be_bytes());

        let block = self.buffer;
        self.compress(&block);

        for i in 0..8 {
            out[i * 4..i * 4 + 4].copy_from_slice(&self.h[i].to_be_bytes());
        }

        Ok(out)
    }

    fn reset(&mut self) -> Result<(), CryptoError> {
        self.h = IV;
        self.buffer_len = 0;
        self.bit_len = 0;
        Ok(())
    }
}
