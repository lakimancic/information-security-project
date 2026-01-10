use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tauri::Emitter;
use crate::crypto::api::progress::ProgressWriter;
use crate::crypto::CryptoRequest;
use crate::crypto::encryptor::Encryptor;
use crate::crypto::errors::CryptoError;

pub fn encrypt_worker(
    app: tauri::AppHandle,
    filename: &str,
    request: &CryptoRequest,
    cancel: Arc<AtomicBool>,
) -> Result<(), CryptoError> {
    use std::fs::File;

    let input = File::open(filename)?;
    let output = File::create(format!("{filename}.enc"))?;
    let total = input.metadata()?.len() as usize;

    app.emit("crypto:start", filename)?;

    let writer = ProgressWriter {
        inner: output,
        processed: 0,
        total,
        filename: format!("{filename}.enc"),
        app: app.clone(),
        cancel,
    };

    let mut encryptor = Encryptor::new(request.clone())?;
    encryptor.encrypt(input, writer)?;

    app.emit("crypto:done", filename)?;
    Ok(())
}
