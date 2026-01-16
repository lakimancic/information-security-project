use std::io::{BufRead, BufReader};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread;
use tauri::Emitter;
use crate::crypto::{CryptoMetadata, CryptoRequest};
use crate::crypto::encryptor::Encryptor;
use crate::crypto::errors::CryptoErrorEvent;
use crate::jobs::{PendingControl, ReceiverRegistry};
use crate::key_manager::key::PlainKey;
use crate::network::errors::NetworkError;
use crate::progress::{CryptoProgress, ProgressWriter};

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
    app.emit("network:recv:pending", metadata.clone())?;

    let (lock, cvar) = &*registry;
    let mut map = lock.lock().unwrap();

    while !map[&addr].approved && !map[&addr].canceled {
        map = cvar.wait(map).unwrap();
    }

    if map[&addr].canceled {
        app.emit("network:recv:denied", metadata.filename.clone())?;
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

    recv_decrypt_worker(app, &output_path, metadata.filename, metadata.size, request, reader, cancel)
}

fn recv_decrypt_worker(
    app: tauri::AppHandle,
    output_file: &Path,
    filename: String,
    total: usize,
    request: CryptoRequest,
    buffered_reader: BufReader<&TcpStream>,
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

    let result = {
        let writer = ProgressWriter {
            inner: tmp_output,
            processed: 0,
            total,
            filename: filename.clone(),
            app: app.clone(),
            cancel: cancel.clone(),
            event: "network:progress".into(),
        };

        let mut encryptor = Encryptor::new(request)?;
        encryptor.decrypt(buffered_reader, writer)
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

    app.emit("network:done", CryptoProgress {
        filename,
        processed: total,
        total,
    })?;

    Ok(())
}