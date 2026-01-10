use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tauri::Emitter;
use crate::crypto::api::progress::ProgressWriter;
use crate::crypto::CryptoRequest;
use crate::crypto::encryptor::Encryptor;
use crate::crypto::errors::CryptoError;

pub fn encrypt_worker(
    app: tauri::AppHandle,
    input_file: &Path,
    output_file: &Path,
    request: &CryptoRequest,
    cancel: Arc<AtomicBool>,
) -> Result<(), CryptoError> {
    use std::fs::File;

    let input = File::open(input_file)?;
    let output = File::create(output_file)?;
    let total = input.metadata()?.len() as usize;

    app.emit("crypto:start", output_file.file_name())?;

    let writer = ProgressWriter {
        inner: output,
        processed: 0,
        total,
        filename: output_file.to_str().unwrap_or_default().to_string(),
        app: app.clone(),
        cancel,
    };

    let mut encryptor = Encryptor::new(request.clone())?;
    encryptor.encrypt(input, writer)?;

    app.emit("crypto:done", output_file.file_name())?;
    Ok(())
}
