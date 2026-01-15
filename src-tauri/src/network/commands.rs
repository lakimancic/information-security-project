use std::net::{IpAddr, SocketAddr};
use crate::AppState;
use crate::crypto::CryptoRequest;
use crate::key_manager::key::PlainKey;
use crate::network::errors::NetworkError;
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
    let jobs = state.jobs.clone();

    let source_explorer = state.source_explorer.lock()
        .map_err(|err| NetworkError::NetworkInternalError(err.to_string()))?;

    let source_path = source_explorer.get_current_path_buf();

    try_start_encrypt_send(app, jobs, source_path, request, file, ip, port)
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