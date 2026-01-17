use std::io::{BufRead, BufReader, Read};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread;
use serde::Serialize;
use tauri::Emitter;
use crate::crypto::{CryptoMetadata, CryptoRequest};
use crate::crypto::encryptor::Encryptor;
use crate::crypto::errors::CryptoErrorEvent;
use crate::crypto::hash_factory::HashFactory;
use crate::hash_wrappers::HashWriter;
use crate::jobs::{PendingControl, ReceiverRegistry};
use crate::key_manager::key::PlainKey;
use crate::network::errors::NetworkError;
use crate::progress::{CryptoProgress, ProgressWriter};


#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PendingFile {
    filename: String,
    sock_addr: SocketAddr,
    size: usize,
}

pub fn spawn_recv_worker(
    app: tauri::AppHandle,
    registry: ReceiverRegistry,
    stream: TcpStream,
    output_path: PathBuf,
    addr: SocketAddr,
    key: PlainKey,
) {
    let cancel = Arc::new(AtomicBool::new(false));

    thread::spawn(move || {
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
    registry: ReceiverRegistry,
    key: PlainKey,
    cancel: Arc<AtomicBool>,
) -> Result<(), NetworkError> {
    let mut reader = BufReader::new(&stream);
    let mut metadata_bytes = Vec::new();

    reader.read_until(b'\0', &mut metadata_bytes)?;
    metadata_bytes.pop();

    let metadata: CryptoMetadata = serde_json::from_slice(&metadata_bytes)?;
    {
        let (lock, _) = &*registry;
        let mut map = lock.lock().unwrap();

        map.insert(
            addr,
            PendingControl {
                approved: false,
                canceled: false,
            },
        );
    }
    app.emit("network:recv:pending", PendingFile {
        filename: metadata.filename.clone(),
        sock_addr: addr,
        size: metadata.size,
    })?;

    let (lock, cvar) = &*registry;
    let mut map = lock.lock().unwrap();

    while !map[&addr].approved && !map[&addr].canceled {
        map = cvar.wait(map).unwrap();
    }

    if map[&addr].canceled {
        app.emit("network:recv:denied", addr)?;
        return Ok(());
    }

    drop(map);

    let mut output_path = output_path.clone();
    output_path.push(&metadata.filename);

    let request = CryptoRequest {
        algorithm: metadata.algorithm,
        mode: metadata.block_mode,
        padding: metadata.padding,
        key: key.key.clone(),
        iv: key.iv.clone(),
    };

    recv_decrypt_worker(app, &output_path, metadata.filename, metadata.size, request, &mut reader, metadata.hash_algo, cancel)
}

fn recv_decrypt_worker(
    app: tauri::AppHandle,
    output_file: &Path,
    filename: String,
    total: usize,
    request: CryptoRequest,
    mut buffered_reader: &mut BufReader<&TcpStream>,
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

    let tmp_output = std::fs::File::create(&tmp_file_path)?;

    let _ = app.emit(
        "network:recv:start",
        CryptoProgress {
            filename: filename.clone(),
            processed: 0,
            total,
        },
    );

    let result: Result<(), NetworkError> = {
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
            Some(hash_algo) => Some(HashFactory::create(&hash_algo)?),
            None => None,
        };
        let mut hash_writer = HashWriter::new(&mut writer, hash_function);

        let mut encryptor = Encryptor::new(request)?;
        let take_size = encryptor.padded_size(total);

        let mut limited_reader = buffered_reader.take(take_size);
        encryptor.decrypt(&mut limited_reader, &mut hash_writer)?;

        if let Some(hash) = hash_writer.finalize_hash() {
            let real_hash = hash?;
            let mut expected_hash = vec![0u8; real_hash.len()];
            buffered_reader.read_exact(&mut expected_hash)?;

            if expected_hash != real_hash {
                return Err(NetworkError::HashVerificationFailed);
            }
        }

        Ok(())
    };

    if let Err(e) = result {
        let _ = std::fs::remove_file(&tmp_file_path);
        return Err(NetworkError::from(e));
    }

    if cancel.load(std::sync::atomic::Ordering::SeqCst) {
        let _ = std::fs::remove_file(&tmp_file_path);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Interrupted,
            "Receiving cancelled",
        )
            .into());
    }

    std::fs::rename(&tmp_file_path, output_file)?;

    app.emit("network:recv:done", CryptoProgress {
        filename,
        processed: total,
        total,
    })?;

    Ok(())
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
