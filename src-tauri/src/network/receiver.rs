use std::io::{BufRead, BufReader, Read};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread;
use tauri::Emitter;
use crate::crypto::{CryptoMetadata, CryptoRequest};
use crate::crypto::encryptor::Encryptor;
use crate::crypto::errors::{CryptoErrorEvent};
use crate::crypto::hash_factory::HashFactory;
use crate::hash_wrappers::{HashReader};
use crate::jobs::{CryptoJob, JobGuard, JobRegistry};
use crate::key_manager::key::PlainKey;
use crate::network::errors::NetworkError;
use crate::progress::{CryptoProgress, ProgressWriter};


pub fn spawn_recv_worker(
    app: tauri::AppHandle,
    registry: JobRegistry,
    stream: TcpStream,
    output_path: PathBuf,
    addr: SocketAddr,
    key: PlainKey,
) {
    thread::spawn(move || {
        let cancel = Arc::new(AtomicBool::new(false));

        let result = recv_worker(
            app.clone(),
            stream,
            addr,
            output_path,
            registry.clone(),
            key,
            cancel.clone(),
        );

        if let Err(err) = result {
            let _ = app.emit(
                "network:recv:error",
                CryptoErrorEvent {
                    err: err.to_string(),
                    filename: addr.to_string(),
                },
            );
        }
    });
}

fn recv_worker(
    app: tauri::AppHandle,
    stream: TcpStream,
    addr: SocketAddr,
    output_path: PathBuf,
    registry: JobRegistry,
    key: PlainKey,
    cancel: Arc<AtomicBool>,
) -> Result<(), NetworkError> {
    let mut reader = BufReader::new(&stream);
    let mut metadata_bytes = Vec::new();

    reader.read_until(b'\0', &mut metadata_bytes)?;
    metadata_bytes.pop();

    let metadata: CryptoMetadata = serde_json::from_slice(&metadata_bytes)?;
    tracing::info!("Receiving file {} from: {}", metadata.filename, addr);
    tracing::info!("Crypto Metadata: {:?}", metadata);

    registry.lock().unwrap().insert(
        metadata.filename.clone(),
        CryptoJob { cancel: cancel.clone() },
    );

    let _guard = JobGuard {
        registry: registry.clone(),
        filename: metadata.filename.clone(),
    };

    if cancel.load(std::sync::atomic::Ordering::SeqCst) {
        return Err(NetworkError::CancelledBeforeStart);
    }

    let mut output_path = output_path.clone();
    output_path.push(&metadata.filename);

    let request = CryptoRequest {
        algorithm: metadata.algorithm,
        mode: metadata.block_mode,
        padding: metadata.padding,
        key: key.key.clone(),
        iv: key.iv.clone(),
    };

    match recv_decrypt_worker(app, &output_path, metadata.filename, metadata.size, request, &mut reader, metadata.hash_algo, cancel) {
        Ok(_) => {
            tracing::info!("Received file successfully decrypted: {}", output_path.display());
            Ok(())
        }
        Err(err) => {
            tracing::error!("Receiving file failed: {}", output_path.display());
            tracing::error!("Receive error: {}", err);
            Err(err)
        }
    }
}

fn recv_decrypt_worker(
    app: tauri::AppHandle,
    output_file: &Path,
    filename: String,
    total: usize,
    request: CryptoRequest,
    buffered_reader: &mut BufReader<&TcpStream>,
    hash_algo: Option<String>,
    cancel: Arc<AtomicBool>,
) -> Result<(), NetworkError> {
    let tmp_file_path = output_file.with_extension(format!(
        "{}.tmp",
        output_file
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("tmp")
    ));

    let result: Result<(), NetworkError> = (|| {
        let tmp_output = std::fs::File::create(&tmp_file_path)?;

        let _ = app.emit(
            "network:recv:start",
            CryptoProgress {
                filename: filename.clone(),
                processed: 0,
                total,
            },
        );

        let mut writer = ProgressWriter {
            inner: tmp_output,
            processed: 0,
            total,
            filename: filename.clone(),
            app: app.clone(),
            cancel: cancel.clone(),
            event: "network:recv:progress".into(),
        };

        let hash_function = match hash_algo {
            Some(algo) => Some(HashFactory::create(&algo)?),
            None => None,
        };

        let mut encryptor = Encryptor::new(request)?;
        let take_size = encryptor.padded_size(total);

        let mut limited_reader = buffered_reader.take(take_size);
        let mut hash_reader = HashReader::new(&mut limited_reader, hash_function);

        encryptor.decrypt(&mut hash_reader, &mut writer)?;

        if let Some(hash) = hash_reader.finalize_hash() {
            let real_hash = hash?;
            let mut expected_hash = vec![0u8; real_hash.len()];
            buffered_reader.read_exact(&mut expected_hash)?;

            if expected_hash != real_hash {
                tracing::warn!("Hash verification failed: {:x?} {:x?}", real_hash, expected_hash);
                return Err(NetworkError::HashVerificationFailed);
            }
        }

        if cancel.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "Receiving cancelled",
            )
                .into());
        }

        std::fs::rename(&tmp_file_path, output_file)?;

        app.emit(
            "network:recv:done",
            CryptoProgress {
                filename,
                processed: total,
                total,
            },
        )?;

        tracing::info!("File successfully received: {}", output_file.display());
        Ok(())
    })();

    if result.is_err() {
        let _ = std::fs::remove_file(&tmp_file_path);
    }

    if let Err(err) = result {
        let _ = app.emit(
            "network:recv:error",
            CryptoErrorEvent {
                err: err.to_string(),
                filename: output_file.file_name().unwrap().to_str().unwrap().to_string(),
            },
        );
        Err(err)
    }
    else {
        result
    }
}


pub fn read_key_from_stream(stream: &mut TcpStream) -> Result<PlainKey, NetworkError> {
    use std::io::{Read};

    const MAX_KEY_BYTES: usize = 4096 / 8;

    let mut len_buf = [0u8; 2];
    stream.read_exact(&mut len_buf)?;
    let key_len = u16::from_le_bytes(len_buf) as usize;

    if key_len == 0 || key_len > MAX_KEY_BYTES {
        return Err(NetworkError::InvalidSocketKey);
    }

    let mut key_bytes = vec![0u8; key_len];
    stream.read_exact(&mut key_bytes)?;

    stream.read_exact(&mut len_buf)?;
    let iv_len = u16::from_le_bytes(len_buf) as usize;

    let iv = if iv_len == 0 {
        None
    } else {
        if iv_len > MAX_KEY_BYTES {
            return Err(NetworkError::InvalidSocketKey);
        }

        let mut iv_bytes = vec![0u8; iv_len];
        stream.read_exact(&mut iv_bytes)?;
        Some(iv_bytes)
    };

    Ok(PlainKey {
        key: key_bytes,
        iv,
    })
}
