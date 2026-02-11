use std::io::{Read, Write};
use std::ops::Deref;

use crate::crypto::{CipherInstance, CryptoRequest};
use crate::crypto::cipher_factory::CipherFactory;
use crate::crypto::errors::CryptoError;

pub struct Encryptor {
    cipher: CipherInstance,
}

const BUFFER_SIZE: usize = 1024 * 1024;

impl Encryptor {
    pub fn new(req: CryptoRequest) -> Result<Self, CryptoError> {
        Ok(Self {
            cipher: CipherFactory::create(req)?,
        })
    }

    pub fn from_instance(instance: CipherInstance) -> Self {
        Self { cipher: instance }
    }

    pub fn encrypt<R, W>(&mut self, mut input: R, mut output: W) -> Result<(), CryptoError>
    where
        R: Read,
        W: Write,
    {
        match self.cipher {
            CipherInstance::Stream(ref mut stream) => {
                let mut buf = vec![0u8; BUFFER_SIZE];
                stream.reset()?;

                loop {
                    let n = input.read(&mut buf)?;
                    if n == 0 {
                        break;
                    }
                    for b in &mut buf[..n] {
                        *b = stream.encrypt_byte(*b)?;
                    }
                    output.write_all(&buf[..n])?;
                }
            }

            CipherInstance::Block {
                ref cipher,
                ref mut mode,
                ref padding,
            } => {
                let bs = cipher.block_size();
                let mut buffer = vec![0u8; BUFFER_SIZE];
                let mut leftover = Vec::<u8>::new();

                mode.reset()?;

                loop {
                    let n = input.read(&mut buffer)?;
                    if n == 0 {
                        break;
                    }

                    leftover.extend_from_slice(&buffer[..n]);

                    let full_len = leftover.len() / bs * bs;
                    let mut blocks = leftover.drain(..full_len).collect::<Vec<_>>();

                    for block in blocks.chunks_exact_mut(bs) {
                        mode.encrypt_next(cipher.deref(), block)?;
                    }

                    output.write_all(&blocks)?;
                }

                if let Some(pad) = padding {
                    pad.pad(&mut leftover, bs)?;
                } else if mode.needs_padding() && !leftover.is_empty() {
                    return Err(CryptoError::UnalignedPlaintext);
                }

                if !leftover.is_empty() {
                    if mode.needs_padding() {
                        for block in leftover.chunks_exact_mut(bs) {
                            mode.encrypt_next(cipher.deref(), block)?;
                        }
                    } else {
                        mode.encrypt_next(cipher.deref(), &mut leftover)?;
                    }

                    output.write_all(&leftover)?;
                }
            }
        }

        Ok(())
    }

    pub fn decrypt<R, W>(&mut self, mut input: R, mut output: W) -> Result<(), CryptoError>
    where
        R: Read,
        W: Write,
    {
        match self.cipher {
            CipherInstance::Stream(ref mut stream) => {
                let mut in_buf = vec![0u8; BUFFER_SIZE];
                let mut out_buf = Vec::<u8>::with_capacity(BUFFER_SIZE);

                stream.reset()?;

                loop {
                    let n = input.read(&mut in_buf)?;
                    if n == 0 {
                        break;
                    }

                    for &b in &in_buf[..n] {
                        out_buf.push(stream.decrypt_byte(b)?);
                    }

                    if out_buf.len() >= BUFFER_SIZE {
                        output.write_all(&out_buf)?;
                        out_buf.clear();
                    }
                }

                if !out_buf.is_empty() {
                    output.write_all(&out_buf)?;
                }
            }

            CipherInstance::Block {
                ref cipher,
                ref mut mode,
                ref padding,
            } => {
                let bs = cipher.block_size();

                let mut in_buf = vec![0u8; BUFFER_SIZE];
                let mut leftover = Vec::<u8>::new();

                let mut last_block: Option<Vec<u8>> = None;
                let mut out_buf = Vec::<u8>::with_capacity(BUFFER_SIZE);

                mode.reset()?;

                loop {
                    let n = input.read(&mut in_buf)?;
                    if n == 0 {
                        break;
                    }

                    leftover.extend_from_slice(&in_buf[..n]);

                    let full_len = (leftover.len() / bs) * bs;
                    let mut blocks = leftover.drain(..full_len).collect::<Vec<_>>();

                    for block in blocks.chunks_exact_mut(bs) {
                        mode.decrypt_next(cipher.deref(), block)?;

                        if let Some(prev) = last_block.replace(block.to_vec()) {
                            out_buf.extend_from_slice(&prev);

                            if out_buf.len() >= BUFFER_SIZE {
                                output.write_all(&out_buf)?;
                                out_buf.clear();
                            }
                        }
                    }
                }

                match (last_block, padding) {
                    (Some(mut block), Some(pad)) => {
                        pad.unpad(&mut block, bs)?;
                        out_buf.extend_from_slice(&block);
                    }

                    (Some(block), None) => {
                        out_buf.extend_from_slice(&block);
                    }

                    (None, None) => {
                        if !leftover.is_empty() {
                            mode.decrypt_next(cipher.deref(), &mut leftover)?;
                            out_buf.extend_from_slice(&leftover);
                        }
                    }

                    (None, Some(_)) => {
                        return Err(CryptoError::InvalidPadding);
                    }
                }

                if !out_buf.is_empty() {
                    output.write_all(&out_buf)?;
                }
            }
        }

        Ok(())
    }

    pub fn padded_size(&self, size: usize) -> u64 {
        match self.cipher {
            CipherInstance::Block {
                ref cipher,
                ref padding,
                ..
            } => {
                if let Some(pad) = padding {
                    (size + pad.pad_size(size, cipher.block_size())) as u64
                } else {
                    size as u64
                }
            }
            _ => size as u64,
        }
    }
}
