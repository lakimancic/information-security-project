use std::net::{IpAddr, SocketAddr, TcpListener};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tauri::Emitter;
use crate::AppState;
use crate::crypto::CryptoRequest;
use crate::crypto::errors::CryptoError;
use crate::key_manager::key::PlainKey;
use crate::network::errors::NetworkError;
use crate::network::receiver::spawn_recv_worker;
use crate::network::sender::{try_send_key, try_start_encrypt_send};

#[tauri::command]
pub async fn send_file(
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
    request: CryptoRequest,
    file: String,
    ip: String,
    port: u16,
) -> Result<(), NetworkError> {
    let jobs = state.send_jobs.clone();

    let source_explorer = state.source_explorer.lock()
        .map_err(|err| NetworkError::NetworkInternalError(err.to_string()))?;

    let source_path = source_explorer.get_current_path_buf();

    try_start_encrypt_send(app, jobs, source_path, request, file, ip, port)
}

#[tauri::command]
pub async fn stop_sending(
    state: tauri::State<'_, AppState>,
    filename: String,
) -> Result<(), NetworkError> {
    let map = state.send_jobs
        .lock()
        .map_err(|e| NetworkError::NetworkInternalError(e.to_string()))?;

    let job = map.get(&filename).ok_or_else(|| NetworkError::FileIsNotSending)?;

    job.cancel.store(true, Ordering::Relaxed);
    Ok(())
}

#[tauri::command]
pub async fn send_key(
    app: tauri::AppHandle,
    key: PlainKey,
    ip: String,
    port: u16,
) -> Result<(), NetworkError> {
    let ip_addr = ip.parse::<IpAddr>()?;
    let socket_addr = SocketAddr::new(ip_addr, port);

    try_send_key(app, &socket_addr, &key)
}

#[tauri::command]
pub fn start_listening(
    state: tauri::State<AppState>,
    app: tauri::AppHandle,
    port: u16,
) -> Result<(), NetworkError> {
    let listener = TcpListener::bind(("0.0.0.0", port))?;
    let stop_flag = Arc::new(AtomicBool::new(false));
    let thread_stop = stop_flag.clone();
    let jobs = state.recv_jobs.clone();

    let destination_explorer = state.dest_explorer.lock()
        .map_err(|err| CryptoError::CryptoInternalError(err.to_string()))?;
    let destination_path = destination_explorer.get_current_path_buf();

    thread::spawn(move || {
        match listener.set_nonblocking(true) {
            Ok(_) => {}
            Err(_) => { return; }
        };

        loop {
            if thread_stop.load(Ordering::SeqCst) {
                break;
            }

            match listener.accept() {
                Ok((stream, addr)) => {
                    spawn_recv_worker(app.clone(), jobs.clone(), stream, destination_path.clone(), addr, PlainKey{
                        key: vec![],
                        iv: None
                    });
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        thread::sleep(Duration::from_millis(100));
                        continue;
                    }
                    else {
                        let _ = app.emit("network:error", e.to_string());
                        break;
                    }
                }
            }
        }
    });

    Ok(())
}