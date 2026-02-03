use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, TcpListener};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use tauri::Emitter;
use crate::AppState;
use crate::crypto::CryptoRequest;
use crate::crypto::errors::CryptoError;
use crate::key_manager::key::PlainKey;
use crate::network::errors::NetworkError;
use crate::network::receiver::{read_key_from_stream, spawn_recv_worker};
use crate::network::sender::{try_send_key, try_start_encrypt_send};

#[tauri::command]
pub async fn send_file(
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
    request: CryptoRequest,
    file: String,
    hash_algo: Option<String>,
    ip: String,
    port: u16,
) -> Result<(), NetworkError> {
    let jobs = state.send_jobs.clone();

    let source_explorer = state.source_explorer.lock()
        .map_err(|err| NetworkError::NetworkInternalError(err.to_string()))?;

    let source_path = source_explorer.get_current_path_buf();

    match try_start_encrypt_send(app, jobs, source_path, request, file, hash_algo, ip, port) {
        Ok(res) => Ok(res),
        Err(e) => {
            tracing::error!("Error sending file: {}", e.to_string());
            Err(e)
        }
    }
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
    key: PlainKey,
    ip: String,
    port: u16,
) -> Result<(), NetworkError> {
    let ip_addr = ip.parse::<IpAddr>()?;
    let socket_addr = SocketAddr::new(ip_addr, port);

    match try_send_key(&socket_addr, &key) {
        Ok(res) => Ok(res),
        Err(e) => {
            tracing::error!("Error sending key: {}", e.to_string());
            Err(e)
        }
    }
}

#[tauri::command]
pub fn start_file_listening(
    state: tauri::State<AppState>,
    app: tauri::AppHandle,
    port: u16,
) -> Result<(), NetworkError> {
    let listener = TcpListener::bind(("0.0.0.0", port))?;
    listener.set_nonblocking(false)?;

    let stop_flag = Arc::new(AtomicBool::new(false));

    {
        let mut ctrl = state.file_listener.lock()
            .map_err(|err| NetworkError::NetworkInternalError(err.to_string()))?;
        ctrl.stop = stop_flag.clone();
    }
    let thread_stop = stop_flag.clone();

    let jobs = state.recv_jobs.clone();

    let destination_explorer = state.dest_explorer.lock()
        .map_err(|err| CryptoError::CryptoInternalError(err.to_string()))?;
    let destination_path = destination_explorer.get_current_path_buf();

    let net_keys = state.net_keys.clone();

    thread::spawn(move || {
        loop {
            if thread_stop.load(Ordering::SeqCst) {
                break;
            }

            match listener.accept() {
                Ok((stream, addr)) => {
                    if thread_stop.load(Ordering::SeqCst) {
                        break;
                    }

                    let locked_net_keys = net_keys.lock().unwrap();
                    let key = locked_net_keys.get(&addr.ip());
                    match key {
                        Some(key) => spawn_recv_worker(app.clone(), jobs.clone(), stream, destination_path.clone(), addr, key.clone()),
                        None => {
                            let _ = app.emit("network:error", NetworkError::SocketKeyNotFound(addr.to_string()).to_string());
                        }
                    }
                }
                Err(e) => {
                    let _ = app.emit("network:error", e.to_string());
                    break;
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub fn stop_file_listening(state: tauri::State<AppState>) {
    if let Ok(ctrl) = state.file_listener.lock() {
        ctrl.stop.store(true, Ordering::SeqCst);
    }
}

#[tauri::command]
pub fn approve_incoming(
    state: tauri::State<AppState>,
    addr: String,
) -> Result<(), NetworkError> {
    let socket: SocketAddr = addr.parse()?;

    let (lock, cvar) = &*state.recv_jobs;

    let mut map = lock
        .lock()
        .map_err(|e| NetworkError::NetworkInternalError(e.to_string()))?;

    if let Some(ctrl) = map.get_mut(&socket) {
        ctrl.approved = true;
    }

    cvar.notify_all();
    Ok(())
}

#[tauri::command]
pub fn deny_incoming(
    state: tauri::State<AppState>,
    addr: String,
) -> Result<(), NetworkError> {
    let socket: SocketAddr = addr.parse()?;

    let (lock, cvar) = &*state.recv_jobs;

    let mut map = lock
        .lock()
        .map_err(|e| NetworkError::NetworkInternalError(e.to_string()))?;

    if let Some(ctrl) = map.get_mut(&socket) {
        ctrl.canceled = true;
    }

    cvar.notify_all();
    Ok(())
}

#[tauri::command]
pub fn start_key_listening(
    state: tauri::State<AppState>,
    app: tauri::AppHandle,
    port: u16,
) -> Result<(), NetworkError> {
    let listener = TcpListener::bind(("0.0.0.0", port))?;
    listener.set_nonblocking(false)?;
    let stop_flag = {
        let mut ctrl = state.key_listener.lock()
            .map_err(|err| NetworkError::NetworkInternalError(err.to_string()))?;
        ctrl.stop = Arc::new(AtomicBool::new(false));
        ctrl.stop.clone()
    };

    let thread_stop = stop_flag.clone();
    let net_keys = state.net_keys.clone();

    thread::spawn(move || {
        loop {
            if thread_stop.load(Ordering::SeqCst) {
                break;
            }

            match listener.accept() {
                Ok((mut stream, addr)) => {
                    if thread_stop.load(Ordering::SeqCst) {
                        break;
                    }

                    match read_key_from_stream(&mut stream) {
                        Ok(key) => {
                            if let Ok(mut map) = net_keys.lock() {
                                map.insert(addr.ip(), key.clone());
                            }

                            tracing::info!("Received key from: {}", addr);
                            let _ = app.emit("network:key:saved", addr.to_string());
                        }
                        Err(err) => {
                            let _ = app.emit("network:key:error", err.to_string());
                        }
                    }

                    break;
                }
                Err(e) => {
                    let _ = app.emit("network:key:error", e.to_string());
                    break;
                }
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub fn stop_key_listening(state: tauri::State<AppState>) {
    if let Ok(ctrl) = state.key_listener.lock() {
        ctrl.stop.store(true, Ordering::SeqCst);
    }
}

#[tauri::command]
pub fn get_network_keys(
    state: tauri::State<AppState>,
) -> Result<HashMap<IpAddr, (usize, usize)>, NetworkError> {
    let net_keys = state.net_keys.lock()
        .map_err(|err| NetworkError::NetworkInternalError(err.to_string()))?;

    let sizes: HashMap<IpAddr, (usize, usize)> = net_keys
        .iter()
        .map(|(ip, key)| {
            let iv_len = key
                .iv
                .as_ref()
                .map(|v| v.len())
                .unwrap_or(0);

            (*ip, (key.key.len(), iv_len))
        })
        .collect();

    Ok(sizes)
}

#[tauri::command]
pub fn remove_network_key(
    state: tauri::State<AppState>,
    ip_addr: IpAddr,
) -> Result<(), NetworkError> {
    let mut net_keys = state.net_keys.lock()
        .map_err(|err| NetworkError::NetworkInternalError(err.to_string()))?;

    net_keys.remove(&ip_addr);

    Ok(())
}
