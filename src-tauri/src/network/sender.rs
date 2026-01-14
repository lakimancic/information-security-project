use std::io::{BufWriter, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use chrono::Utc;
use tauri::Emitter;
use std::thread;
use crate::crypto::api::jobs::{CryptoJob, JobRegistry};
use crate::crypto::{CryptoMetadata, CryptoRequest};
use crate::crypto::encryptor::Encryptor;
use crate::crypto::errors::{CryptoError, CryptoErrorEvent};
use crate::network::errors::NetworkError;
use crate::progress::{CryptoProgress, ProgressWriter};

pub fn try_start_encrypt_send(
    app: tauri::AppHandle,
    jobs: JobRegistry,
    source_path: PathBuf,
    request: CryptoRequest,
    filename: String,
    ip: String,
    port: u16,
) -> Result<(), NetworkError> {
    let mut map = jobs
        .lock()
        .map_err(|e| CryptoError::CryptoInternalError(e.to_string()))?;

    if map.contains_key(&filename) {
        return Ok(());
    }

    let cancel = Arc::new(AtomicBool::new(false));

    map.insert(
        filename.clone(),
        CryptoJob {
            cancel: cancel.clone(),
        },
    );

    drop(map);

    let mut input_file = source_path.clone();
    input_file.push(filename.clone());

    let ip_addr = ip.parse::<IpAddr>()?;
    let socket_addr = SocketAddr::new(ip_addr, port);

    thread::spawn(move || {
        let result = send_worker(app.clone(), &input_file, &socket_addr, &request, cancel);

        jobs.lock().unwrap().remove(&filename);

        if let Err(err) = result {
            let _ = app.emit("network:send_error", CryptoErrorEvent {
                err: err.to_string(),
                filename,
            });
        }
    });

    Ok(())
}

fn send_worker(
    app: tauri::AppHandle,
    input_file: &Path,
    addr: &SocketAddr,
    request: &CryptoRequest,
    cancel: Arc<AtomicBool>,
) -> Result<(), NetworkError> {
    use std::fs::File;

    let input = File::open(input_file)?;
    let total = input.metadata()?.len() as usize;

    let tcp_stream = TcpStream::connect(addr)?;
    tcp_stream.set_nodelay(true)?;

    let mut buf_writer = BufWriter::new(tcp_stream);

    let filename = input_file
        .file_stem()
        .unwrap_or_default()
        .to_str()
        .unwrap_or("")
        .to_string();

    let metadata = CryptoMetadata {
        filename: filename.clone(),
        size: total,
        created: Utc::now().to_string(),
        algorithm: request.algorithm.clone(),
        block_mode: request.mode.clone(),
        hash_algo: None,
        padding: request.padding.clone(),
    };

    let mut metadata_bytes = serde_json::to_vec(&metadata)?;
    metadata_bytes.push(0);

    buf_writer.write_all(&metadata_bytes)?;

    let writer = ProgressWriter {
        inner: buf_writer,
        processed: 0,
        total,
        filename: filename.clone(),
        app: app.clone(),
        cancel: cancel.clone(),
        event: "network:send_progress".into()
    };

    let mut encryptor = Encryptor::new(request.clone())?;

    let _ = app.emit(
        "network:send_start",
        CryptoProgress {
            filename: filename.clone(),
            processed: 0,
            total,
        },
    );

    encryptor.encrypt(input, writer)?;

    let _ = app.emit(
        "network:send_done",
        CryptoProgress {
            filename,
            processed: total,
            total,
        },
    );

    Ok(())
}