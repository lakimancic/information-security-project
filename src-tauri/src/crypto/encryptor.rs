use std::io::{Read, Write};
use std::ops::Deref;
use crate::crypto::{CipherInstance, CryptoRequest};
use crate::crypto::cipher_factory::CipherFactory;
use crate::crypto::errors::CryptoError;

pub struct Encryptor {
    cipher: CipherInstance
}

impl Encryptor {
    pub fn new(req: CryptoRequest) -> Result<Self, CryptoError> {
        Ok(Self {
            cipher: CipherFactory::create(req)?
        })
    }

    pub fn from_instance(instance: CipherInstance) -> Self {
        Self {
            cipher: instance
        }
    }

    pub fn encrypt<R, W>(&mut self, mut input: R, mut output: W) -> Result<(), CryptoError>
    where
        R: Read,
        W: Write
    {
        if let CipherInstance::Stream(ref mut stream_cipher) = self.cipher {
            let mut buffer = vec![0u8; 4096];

            stream_cipher.reset()?;
            loop {
                let n = input.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                for b in &mut buffer[..n] {
                    *b = stream_cipher.encrypt_byte(*b)?
                }
                output.write_all(&buffer[..n])?;
            }
        }
        else if let CipherInstance::Block { ref cipher, ref mut mode, ref padding } = self.cipher {
            let bs = cipher.block_size();
            let chunk_size = (4096 / bs) * bs;

            let mut buffer = vec![0u8; chunk_size];
            let mut leftover = Vec::<u8>::new();

            mode.reset()?;

            loop {
                let n = input.read(&mut buffer)?;
                if n == 0 {
                    break;
                }

                let mut data = if leftover.is_empty() {
                    buffer[..n].to_vec()
                } else {
                    let mut tmp = leftover;
                    tmp.extend_from_slice(&buffer[..n]);
                    leftover = Vec::new();
                    tmp
                };

                let full_len = data.len() / bs * bs;
                let (blocks, rest) = data.split_at_mut(full_len);

                for block in blocks.chunks_exact_mut(bs) {
                    mode.encrypt_next(cipher.deref(), block)?;
                }

                output.write_all(blocks)?;
                leftover.extend_from_slice(rest);
            }

            padding.pad(&mut leftover, bs)?;

            for block in leftover.chunks_exact_mut(bs) {
                mode.encrypt_next(cipher.deref(), block)?;
            }

            output.write_all(&leftover)?;
        }

        Ok(())
    }

    pub fn decrypt<R, W>(&mut self, mut input: R, mut output: W) -> Result<(), CryptoError>
    where
        R: Read,
        W: Write,
    {
        if let CipherInstance::Stream(ref mut stream_cipher) = self.cipher {
            let mut buffer = vec![0u8; 4096];

            stream_cipher.reset()?;
            loop {
                let n = input.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                for b in &mut buffer[..n] {
                    *b = stream_cipher.decrypt_byte(*b)?;
                }
                output.write_all(&buffer[..n])?;
            }
        }
        else if let CipherInstance::Block { ref cipher, ref mut mode, ref padding } = self.cipher {
            let bs = cipher.block_size();
            let chunk_size = (4096 / bs) * bs;

            let mut buffer = vec![0u8; chunk_size];
            let mut leftover = Vec::<u8>::new();
            let mut last_block: Option<Vec<u8>> = None;

            mode.reset()?;

            loop {
                let n = input.read(&mut buffer)?;
                if n == 0 {
                    break;
                }

                let mut data = if leftover.is_empty() {
                    buffer[..n].to_vec()
                } else {
                    let mut tmp = leftover;
                    tmp.extend_from_slice(&buffer[..n]);
                    leftover = Vec::new();
                    tmp
                };

                let full_len = data.len() / bs * bs;
                let (blocks, rest) = data.split_at_mut(full_len);

                for block in blocks.chunks_exact_mut(bs) {
                    mode.decrypt_next(cipher.deref(), block)?;

                    if let Some(prev) = last_block.replace(block.to_vec()) {
                        output.write_all(&prev)?;
                    }
                }

                leftover.extend_from_slice(rest);
            }

            let mut final_block = last_block.ok_or(CryptoError::InvalidPadding)?;

            padding.unpad(&mut final_block, bs)?;
            output.write_all(&final_block)?;
        }

        Ok(())
    }

}