use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use notify::{Watcher};
use serde::Deserialize;
use tauri::Emitter;
use crate::crypto::api::jobs::{try_start_decrypt, try_start_encrypt, JobRegistry};
use crate::crypto::CryptoRequest;
use crate::crypto::errors::CryptoErrorEvent;
use crate::files::errors::FilesError;
use crate::key_manager::key::PlainKey;

pub struct WatcherService {
    pub(crate) stop: Arc<AtomicBool>,
    pub(crate) handle: std::thread::JoinHandle<()>,
}

#[derive(Deserialize, Clone)]
pub enum WatchMode {
    Encrypt(CryptoRequest),
    Decrypt(PlainKey),
}

pub type WatcherState = Arc<Mutex<Option<WatcherService>>>;

pub fn start_watcher(
    watch_path: PathBuf,
    output_path: PathBuf,
    app: tauri::AppHandle,
    jobs: JobRegistry,
    mode: WatchMode,
) -> WatcherService {
    let stop = Arc::new(AtomicBool::new(false));
    let stop_thread = stop.clone();

    let handle = std::thread::spawn(move || {
        use notify::{EventKind};
        use std::sync::mpsc::channel;

        let (tx, rx) = channel();

        let watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        });

        if let Err(err) = watcher {
            let _ = app.emit("fsw:error", err.to_string());
            return;
        }
        let mut watcher = watcher.unwrap();

        let result = watcher.watch(&watch_path, notify::RecursiveMode::NonRecursive);

        if let Err(err) = result {
            let _ = app.emit("fsw:error", err.to_string());
            return;
        }

        while !stop_thread.load(Ordering::SeqCst) {
            if let Ok(Ok(event)) = rx.recv_timeout(std::time::Duration::from_millis(200)) {
                if matches!(event.kind, EventKind::Create(_)) {
                    for path in event.paths {
                        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                            match &mode {
                                WatchMode::Encrypt(req) => {
                                    if matches!(path.extension().and_then(|s| s.to_str()), Some("enc" | "tmp")) {
                                        continue;
                                    }
                                    let _ = try_start_encrypt(
                                        app.clone(),
                                        jobs.clone(),
                                        watch_path.clone(),
                                        output_path.clone(),
                                        name.to_string(),
                                        req.clone(),
                                    );
                                }
                                WatchMode::Decrypt(key) => {
                                    if path.extension().and_then(|s| s.to_str()) == Some("enc") {
                                        let _ = try_start_decrypt(
                                            app.clone(),
                                            jobs.clone(),
                                            watch_path.clone(),
                                            output_path.clone(),
                                            name.to_string(),
                                            key.clone(),
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    WatcherService { stop, handle }
}
