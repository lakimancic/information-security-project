use std::sync::atomic::Ordering;
use crate::AppState;
use crate::crypto::api::jobs::{try_start_decrypt, try_start_encrypt};
use crate::crypto::CryptoRequest;
use crate::crypto::errors::CryptoError;
use crate::key_manager::key::PlainKey;

#[tauri::command]
pub async fn encrypt_file(
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
    request: CryptoRequest,
    file: String,
) -> Result<(), CryptoError> {
    let jobs = state.jobs.clone();

    let source_explorer = state.source_explorer.lock()
        .map_err(|err| CryptoError::CryptoInternalError(err.to_string()))?;

    let destination_explorer = state.dest_explorer.lock()
        .map_err(|err| CryptoError::CryptoInternalError(err.to_string()))?;

    let source_path = source_explorer.get_current_path_buf();
    let destination_path = destination_explorer.get_current_path_buf();

    try_start_encrypt(app, jobs, source_path, destination_path, file, request)
}

#[tauri::command]
pub async fn decrypt_file(
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
    key: PlainKey,
    file: String,
) -> Result<(), CryptoError> {
    let jobs = state.jobs.clone();

    let source_explorer = state.source_explorer.lock()
        .map_err(|err| CryptoError::CryptoInternalError(err.to_string()))?;

    let destination_explorer = state.dest_explorer.lock()
        .map_err(|err| CryptoError::CryptoInternalError(err.to_string()))?;

    let source_path = source_explorer.get_current_path_buf();
    let destination_path = destination_explorer.get_current_path_buf();

    try_start_decrypt(app, jobs, source_path, destination_path, file, key)
}

#[tauri::command]
pub async fn stop_processing(
    state: tauri::State<'_, AppState>,
    filename: String,
    encrypt: bool,
) -> Result<(), CryptoError> {
    let map = state.jobs
        .lock()
        .map_err(|e| CryptoError::CryptoInternalError(e.to_string()))?;

    let job = map.get(&filename).ok_or_else(|| {
        CryptoError::FileIsNotProcessing(if encrypt {
            "encrypting".into()
        } else {
            "decrypting".into()
        })
    })?;

    job.cancel.store(true, Ordering::Relaxed);
    Ok(())
}