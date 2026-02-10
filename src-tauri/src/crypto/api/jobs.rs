use std::{sync::{Arc}, thread};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool};
use tauri::Emitter;
use crate::crypto::api::worker::{decrypt_worker, encrypt_worker};
use crate::crypto::CryptoRequest;
use crate::crypto::errors::{CryptoError, CryptoErrorEvent};
use crate::jobs::{CryptoJob, JobRegistry};
use crate::key_manager::key::PlainKey;


fn try_start_job<F>(
    app: tauri::AppHandle,
    jobs: JobRegistry,
    source_path: PathBuf,
    destination_path: PathBuf,
    filename: String,
    spawn: F,
) -> Result<(), CryptoError>
where
    F: FnOnce(tauri::AppHandle, JobRegistry, PathBuf, PathBuf, String, Arc<AtomicBool>),
{
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

    spawn(app, jobs, source_path, destination_path, filename, cancel);

    Ok(())
}

fn spawn_crypto_worker<P, W>(
    app: tauri::AppHandle,
    jobs: JobRegistry,
    input_path: PathBuf,
    output_path: PathBuf,
    filename: String,
    payload: P,
    cancel: Arc<AtomicBool>,
    make_output_name: fn(&str) -> String,
    worker: W,
)
where
    P: Send + 'static,
    W: FnOnce(tauri::AppHandle, &PathBuf, &PathBuf, P, Arc<AtomicBool>) -> Result<(), CryptoError>
    + Send
    + 'static,
{
    let mut input_file = input_path.clone();
    input_file.push(filename.clone());

    let mut output_file = output_path.clone();
    output_file.push(make_output_name(&filename));

    thread::spawn(move || {
        let result = worker(app.clone(), &input_file, &output_file, payload, cancel);

        jobs.lock().unwrap().remove(&filename);

        if let Err(err) = result {
            let _ = app.emit("crypto:error", CryptoErrorEvent {
                err: err.to_string(),
                filename: make_output_name(&filename),
            });
        }
    });
}

fn spawn_encrypt_worker(
    app: tauri::AppHandle,
    jobs: JobRegistry,
    input_path: &PathBuf,
    output_path: &PathBuf,
    filename: String,
    request: CryptoRequest,
    cancel: Arc<AtomicBool>,
) {
    spawn_crypto_worker(
        app, jobs, input_path.clone(), output_path.clone(), filename, request, cancel,
        |name| format!("{name}.enc"),
        |app, in_file, out_file, req, cancel| {
            encrypt_worker(app, in_file.as_ref(), out_file.as_ref(), &req, cancel)
        },
    );
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
    spawn_crypto_worker(
        app, jobs, input_path.clone(), output_path.clone(), filename, key, cancel,
        |name| name.strip_suffix(".enc").unwrap_or(name).to_string(),
        |app, in_file, out_file, key, cancel| {
            decrypt_worker(app, in_file.as_ref(), out_file.as_ref(), &key, cancel)
        },
    );
}

pub fn try_start_encrypt(
    app: tauri::AppHandle,
    jobs: JobRegistry,
    source_path: PathBuf,
    destination_path: PathBuf,
    filename: String,
    request: CryptoRequest,
) -> Result<(), CryptoError> {
    try_start_job(app, jobs, source_path, destination_path, filename,
    move |app, jobs, src, dst, name, cancel| {
        spawn_encrypt_worker(app, jobs, &src, &dst, name, request, cancel)
    })
}

pub fn try_start_decrypt(
    app: tauri::AppHandle,
    jobs: JobRegistry,
    source_path: PathBuf,
    destination_path: PathBuf,
    filename: String,
    key: PlainKey,
) -> Result<(), CryptoError> {
    try_start_job(app, jobs, source_path, destination_path, filename,
    move |app, jobs, src, dst, name, cancel| {
        spawn_decrypt_worker(app, jobs, &src, &dst, name, key, cancel)
    })
}
