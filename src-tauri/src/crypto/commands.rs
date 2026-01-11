use std::sync::atomic::Ordering;
use crate::AppState;
use crate::crypto::api::jobs::{try_start_encrypt, JobRegistry};
use crate::crypto::CryptoRequest;
use crate::crypto::errors::CryptoError;

#[tauri::command]
pub async fn encrypt_file(
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
    request: CryptoRequest,
    file: String,
) -> Result<(), CryptoError> {
    let jobs = state.jobs.clone();
    try_start_encrypt(state, app, jobs, file, request)
}

#[tauri::command]
pub async fn stop_encryption(
    state: tauri::State<'_, AppState>,
    filename: String,
) -> Result<(), CryptoError> {
    let map = state.jobs
        .lock()
        .map_err(|e| CryptoError::CryptoInternalError(e.to_string()))?;

    let job = map.get(&filename).ok_or_else(|| {
        CryptoError::FileIsNotEncrypting
    })?;

    job.cancel.store(true, Ordering::Relaxed);
    Ok(())
}