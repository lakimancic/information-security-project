use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpmc::channel;
use notify::{EventKind, Watcher};
use crate::AppState;
use crate::crypto::api::jobs::{try_start_decrypt, try_start_encrypt, JobRegistry};
use crate::crypto::CryptoRequest;
use crate::crypto::errors::CryptoError;
use crate::files::errors::FilesError;
use crate::key_manager::key::PlainKey;

pub struct WatcherService {
    pub(crate) stop: Arc<AtomicBool>,
    pub(crate) handle: std::thread::JoinHandle<()>,
}

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

        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        }).unwrap();

        watcher.watch(&watch_path, notify::RecursiveMode::NonRecursive).unwrap();

        while !stop_thread.load(Ordering::SeqCst) {
            if let Ok(Ok(event)) = rx.recv_timeout(std::time::Duration::from_millis(200)) {
                if matches!(event.kind, EventKind::Create(_)) {
                    for path in event.paths {
                        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                            match &mode {
                                WatchMode::Encrypt(req) => {
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
    });

    WatcherService { stop, handle }
}
