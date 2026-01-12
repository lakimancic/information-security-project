use std::{collections::HashMap, sync::{Arc, Mutex}, thread};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Emitter;
use crate::AppState;
use crate::crypto::api::worker::{decrypt_worker, encrypt_worker};
use crate::crypto::CryptoRequest;
use crate::crypto::errors::{CryptoError, CryptoErrorEvent};
use crate::key_manager::key::PlainKey;

pub struct CryptoJob {
    pub cancel: Arc<AtomicBool>,
}

pub type JobRegistry = Arc<Mutex<HashMap<String, CryptoJob>>>;

fn spawn_encrypt_worker(
    app: tauri::AppHandle,
    jobs: JobRegistry,
    input_path: &PathBuf,
    output_path: &PathBuf,
    filename: String,
    request: CryptoRequest,
    cancel: Arc<AtomicBool>,
) {
    let mut input_file = input_path.clone();
    input_file.push(filename.clone());
    let mut output_file = output_path.clone();
    output_file.push(format!("{filename}.enc"));

    thread::spawn(move || {
        let result = encrypt_worker(app.clone(), input_file.as_ref(), output_file.as_ref(), &request, cancel);
        
        jobs.lock().unwrap().remove(&filename);

        if let Err(err) = result {
            let _ = app.emit("crypto:error", err.to_string());
        }
    });
}

pub fn try_start_encrypt(
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
    jobs: JobRegistry,
    filename: String,
    request: CryptoRequest,
) -> Result<(), CryptoError> {
    let mut map = jobs
        .lock()
        .map_err(|e| CryptoError::CryptoInternalError(e.to_string()))?;

    let source_explorer = state.source_explorer.lock()
        .map_err(|e| CryptoError::CryptoInternalError(e.to_string()))?;

    let destination_explorer = state.dest_explorer.lock()
        .map_err(|e| CryptoError::CryptoInternalError(e.to_string()))?;

    let source_path = source_explorer.get_current_path_buf();
    let destination_path = destination_explorer.get_current_path_buf();

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
    spawn_encrypt_worker(app, jobs, &source_path, &destination_path, filename, request, cancel);

    Ok(())
}

fn spawn_decrypt_worker(
    app: tauri::AppHandle,
    jobs: JobRegistry,
    input_path: &PathBuf,
    output_path: &PathBuf,
    filename: String,
    key: PlainKey,
    cancel: Arc<AtomicBool>,
) {
    let mut input_file = input_path.clone();
    input_file.push(filename.clone());
    let mut output_file = output_path.clone();
    output_file.push(filename.strip_suffix(".enc").unwrap_or(&filename));

    thread::spawn(move || {
        let result = decrypt_worker(app.clone(), input_file.as_ref(), output_file.as_ref(), &key, cancel);

        jobs.lock().unwrap().remove(&filename);

        if let Err(err) = result {
            eprintln!("{}", err);
            let _ = app.emit("crypto:error", CryptoErrorEvent {
                err: err.to_string(),
                filename: filename.clone(),
            });
        }
    });
}

pub fn try_start_decrypt(
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
    jobs: JobRegistry,
    filename: String,
    key: PlainKey,
) -> Result<(), CryptoError> {
    let mut map = jobs
        .lock()
        .map_err(|e| CryptoError::CryptoInternalError(e.to_string()))?;

    let source_explorer = state.source_explorer.lock()
        .map_err(|e| CryptoError::CryptoInternalError(e.to_string()))?;

    let destination_explorer = state.dest_explorer.lock()
        .map_err(|e| CryptoError::CryptoInternalError(e.to_string()))?;

    let source_path = source_explorer.get_current_path_buf();
    let destination_path = destination_explorer.get_current_path_buf();

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
    spawn_decrypt_worker(app, jobs, &source_path, &destination_path, filename, key, cancel);

    Ok(())
}