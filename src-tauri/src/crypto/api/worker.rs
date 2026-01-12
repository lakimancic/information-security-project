use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use chrono::{DateTime, Utc};
use tauri::Emitter;
use crate::crypto::api::progress::{CryptoProgress, ProgressWriter};
use crate::crypto::{CryptoMetadata, CryptoRequest};
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

    let tmp_file_path = output_file.with_extension(format!(
        "{}.tmp",
        output_file.extension().and_then(|s| s.to_str()).unwrap_or("tmp")
    ));

    let input = File::open(&input_file)?;
    let mut tmp_output = File::create(&tmp_file_path)?;
    let total = input.metadata()?.len() as usize;

    let output_str = output_file.file_name().unwrap_or_default().to_str().unwrap_or("").to_string();
    let input_str = input_file.file_stem().unwrap_or_default().to_str().unwrap_or("").to_string();

    let datetime: DateTime<Utc> = Utc::now();

    let metadata = CryptoMetadata{
        filename: input_str,
        size: total,
        created: datetime.to_string(),
        algorithm: request.algorithm.clone(),
        block_mode: request.mode.clone(),
        hash_algo: None,
    };

    let metadata_str = serde_json::to_string(&metadata)?;
    let mut metadata_bytes = metadata_str.as_bytes().to_vec();
    metadata_bytes.push(0);

    tmp_output.write_all(&metadata_bytes)?;

    app.emit("crypto:start", CryptoProgress {
        filename: output_str.clone(),
        processed: 0,
        total,
    })?;

    let result = {
        let writer = ProgressWriter {
            inner: tmp_output,
            processed: 0,
            total,
            filename: output_str.clone(),
            app: app.clone(),
            cancel: cancel.clone()
        };

        let mut encryptor = Encryptor::new(request.clone())?;
        encryptor.encrypt(input, writer)
    };

    if let Err(e) = result {
        let _ = std::fs::remove_file(&tmp_file_path)?;
        return Err(e.into());
    }

    if cancel.load(std::sync::atomic::Ordering::SeqCst) {
        let _ = std::fs::remove_file(&tmp_file_path)?;
        return Err(std::io::Error::new(
            std::io::ErrorKind::Interrupted,
            "Encryption cancelled",
        ).into());
    }

    std::fs::rename(&tmp_file_path, &output_file)?;

    app.emit("crypto:done", CryptoProgress {
        filename: output_str,
        processed: total,
        total,
    })?;

    Ok(())
}
