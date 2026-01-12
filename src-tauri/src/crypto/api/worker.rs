use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use chrono::{DateTime, Utc};
use tauri::Emitter;
use crate::crypto::api::progress::{CryptoProgress, ProgressWriter};
use crate::crypto::{CryptoMetadata, CryptoRequest};
use crate::crypto::encryptor::Encryptor;
use crate::crypto::errors::CryptoError;
use crate::key_manager::key::PlainKey;

fn run_crypto_job<F>(
    app: tauri::AppHandle,
    output_file: &Path,
    total: usize,
    filename: String,
    cancel: Arc<AtomicBool>,
    run: F,
) -> Result<(), CryptoError>
where
    F: FnOnce(ProgressWriter<std::fs::File>) -> Result<(), CryptoError>,
{
    let tmp_file_path = output_file.with_extension(format!(
        "{}.tmp",
        output_file
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("tmp")
    ));

    let tmp_output = std::fs::File::create(&tmp_file_path)?;

    let result = {
        let writer = ProgressWriter {
            inner: tmp_output,
            processed: 0,
            total,
            filename: filename.clone(),
            app: app.clone(),
            cancel: cancel.clone(),
        };

        run(writer)
    };

    if let Err(e) = result {
        let _ = std::fs::remove_file(&tmp_file_path);
        return Err(e);
    }

    if cancel.load(std::sync::atomic::Ordering::SeqCst) {
        let _ = std::fs::remove_file(&tmp_file_path);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Interrupted,
            "Encryption cancelled",
        )
            .into());
    }

    std::fs::rename(&tmp_file_path, output_file)?;

    app.emit("crypto:done", CryptoProgress {
        filename,
        processed: total,
        total,
    })?;

    Ok(())
}

pub fn encrypt_worker(
    app: tauri::AppHandle,
    input_file: &Path,
    output_file: &Path,
    request: &CryptoRequest,
    cancel: Arc<AtomicBool>,
) -> Result<(), CryptoError> {
    use std::fs::File;

    let input = File::open(input_file)?;
    let total = input.metadata()?.len() as usize;

    let output_str = output_file
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("")
        .to_string();

    let input_str = input_file
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("")
        .to_string();

    let metadata = CryptoMetadata {
        filename: input_str,
        size: total,
        created: Utc::now().to_string(),
        algorithm: request.algorithm.clone(),
        block_mode: request.mode.clone(),
        hash_algo: None,
        padding: request.padding.clone(),
    };

    let mut metadata_bytes = serde_json::to_vec(&metadata)?;
    metadata_bytes.push(0);

    app.emit("crypto:start", CryptoProgress {
        filename: output_str.clone(),
        processed: 0,
        total,
    })?;

    run_crypto_job(
        app,
        output_file,
        total,
        output_str,
        cancel,
        |writer| {
            use std::io::Write;
            let mut writer = writer;

            writer.inner.write_all(&metadata_bytes)?;

            let mut encryptor = Encryptor::new(request.clone())?;
            encryptor.encrypt(input, writer)
        },
    )
}

pub fn decrypt_worker(
    app: tauri::AppHandle,
    input_file: &Path,
    output_file: &Path,
    key: &PlainKey,
    cancel: Arc<AtomicBool>,
) -> Result<(), CryptoError> {
    use std::fs::File;

    let input = File::open(input_file)?;
    let output_str = output_file
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("")
        .to_string();

    let mut buffered_input = BufReader::new(input);
    let mut metadata_bytes = Vec::new();

    buffered_input.read_until(b'\0', &mut metadata_bytes)?;
    metadata_bytes.pop();

    let metadata: CryptoMetadata = serde_json::from_slice(&metadata_bytes)?;

    let request = CryptoRequest {
        algorithm: metadata.algorithm,
        mode: metadata.block_mode,
        key: key.key.clone(),
        iv: key.iv.clone(),
        padding: metadata.padding,
    };

    let total = metadata.size;

    run_crypto_job(
        app,
        output_file,
        total,
        output_str,
        cancel,
        |writer| {
            let mut encryptor = Encryptor::new(request)?;
            encryptor.decrypt(buffered_input, writer)
        },
    )
}
