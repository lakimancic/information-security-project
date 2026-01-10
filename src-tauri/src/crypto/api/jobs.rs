use std::{collections::HashMap, sync::{Arc, Mutex}, thread};

use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Emitter;
use crate::crypto::api::worker::encrypt_worker;
use crate::crypto::CryptoRequest;
use crate::crypto::errors::CryptoError;

pub struct CryptoJob {
    pub cancel: Arc<AtomicBool>,
}

pub type JobRegistry = Arc<Mutex<HashMap<String, CryptoJob>>>;

fn spawn_encrypt_worker(
    app: tauri::AppHandle,
    jobs: JobRegistry,
    filename: String,
    request: CryptoRequest,
    cancel: Arc<AtomicBool>,
) {
    thread::spawn(move || {
        let result = encrypt_worker(app.clone(), &filename, &request, cancel);
        
        jobs.lock().unwrap().remove(&filename);

        if let Err(err) = result {
            let _ = app.emit("crypto:error", err.to_string());
        }
    });
}

fn try_start_encrypt(
    app: tauri::AppHandle,
    jobs: JobRegistry,
    filename: String,
    request: CryptoRequest,
) -> Result<(), CryptoError> {
    let mut map = jobs
        .lock()
        .map_err(|_| CryptoError::CryptoInternalError("Lock poisoned".into()))?;

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
    spawn_encrypt_worker(app, jobs, filename, request, cancel);

    Ok(())
}
