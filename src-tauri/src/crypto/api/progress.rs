use std::io::{Result, Write};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter};

#[derive(Clone, serde::Serialize)]
pub struct CryptoProgress {
    pub filename: String,
    pub processed: usize,
    pub total: usize,
}

pub struct ProgressWriter<W> {
    pub(crate) inner: W,
    pub(crate) processed: usize,
    pub(crate) total: usize,
    pub(crate) filename: String,
    pub(crate) app: AppHandle,
    pub(crate) cancel: Arc<AtomicBool>,
}

impl<W> ProgressWriter<W> {
    pub fn new(inner: W, filename: String, total: usize, app: AppHandle) -> Self {
        Self {
            inner,
            processed: 0,
            total,
            filename,
            app,
            cancel: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl<W: Write> Write for ProgressWriter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if self.cancel.load(Ordering::Relaxed) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "Encryption cancelled",
            ));
        }

        let written = self.inner.write(buf)?;
        self.processed += written;

        let _ = self.app.emit(
            "crypto:progress",
            CryptoProgress {
                processed: self.processed,
                total: self.total,
                filename: self.filename.clone(),
            },
        );

        Ok(written)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
