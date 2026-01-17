use std::io;
use std::io::{Read, Write};
use crate::crypto::errors::CryptoError;
use crate::crypto::hash::HashFunction;

pub struct HashWriter<W: Write> {
    inner: W,
    hasher: Option<Box<dyn HashFunction>>,
}

impl<W: Write> HashWriter<W> {
    pub fn new(inner: W, hasher: Option<Box<dyn HashFunction>>) -> Self {
        Self { inner, hasher }
    }

    pub fn into_inner(self) -> W {
        self.inner
    }

    pub fn finalize_hash(&mut self) -> Option<Result<Vec<u8>, CryptoError>> {
        self.hasher.as_mut().map(|h| h.finalize())
    }
}

impl<W: Write> Write for HashWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.inner.write(buf)?;

        if let Some(hasher) = &mut self.hasher {
            hasher.update(&buf[..n]).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, e.to_string())
            })?;
        }

        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

pub struct HashReader<R: Read> {
    inner: R,
    hasher: Option<Box<dyn HashFunction>>,
}

impl<R: Read> HashReader<R> {
    pub fn new(inner: R, hasher: Option<Box<dyn HashFunction>>) -> Self {
        Self { inner, hasher }
    }

    pub fn into_inner(self) -> R {
        self.inner
    }

    pub fn finalize_hash(&mut self) -> Option<Result<Vec<u8>, CryptoError>> {
        self.hasher.as_mut().map(|h| h.finalize())
    }
}

impl<R: Read> Read for HashReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.inner.read(buf)?;

        if n > 0 {
            if let Some(hasher) = &mut self.hasher {
                hasher.update(&buf[..n]).map_err(|e| {
                    io::Error::new(io::ErrorKind::Other, e.to_string())
                })?;
            }
        }

        Ok(n)
    }
}