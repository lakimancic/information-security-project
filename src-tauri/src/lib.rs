#![feature(mpmc_channel)]

mod files;
pub mod crypto;
mod key_manager;
mod network;
mod progress;
mod jobs;
pub mod hash_wrappers;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, prelude::*};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use crate::crypto::commands::{decrypt_file, encrypt_file, stop_processing};
use crate::files::commands::{change_dir, get_files, go_dir_back, set_current_dir};
use crate::files::commands::{start_file_watching, stop_file_watching};
use crate::key_manager::commands::{find_key, find_keys_by_algo, generate_new_key, list_keys, remove_key,
                                   load_keys, save_keys};
use crate::network::commands::{send_file, send_key, start_file_listening, stop_file_listening, approve_incoming, deny_incoming,
                               start_key_listening, stop_key_listening, get_network_keys, remove_network_key, stop_sending, stop_receiving};
use crate::files::file_explorer::FileExplorer;
use crate::files::watch::WatcherState;
use crate::jobs::{JobRegistry, ListenerControl, ReceiverRegistry};
use crate::key_manager::key_manager::KeyManager;
use crate::network::NetworkKeys;

pub struct AppState {
    jobs: JobRegistry,
    send_jobs: JobRegistry,
    recv_jobs: ReceiverRegistry,
    file_listener: Mutex<ListenerControl>,
    key_listener: Mutex<ListenerControl>,
    source_explorer: Mutex<FileExplorer>,
    dest_explorer: Mutex<FileExplorer>,
    key_manager: Mutex<KeyManager>,
    net_keys: Arc<Mutex<NetworkKeys>>,
    watcher: WatcherState
}

fn init_tracing() -> WorkerGuard {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let file_appender =
        RollingFileAppender::new(Rotation::NEVER, "logs", &format!("app-{}.log", today));
    let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);

    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_timer(fmt::time::LocalTime::rfc_3339())
        .with_ansi(true);

    let file_layer = fmt::layer()
        .with_writer(non_blocking_file)
        .with_timer(fmt::time::LocalTime::rfc_3339());

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    guard
}

#[allow(clippy::missing_panics_doc)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _tracing_guard = init_tracing();

    tauri::Builder::default()
        .manage(AppState {
            source_explorer: Mutex::new(FileExplorer::new()),
            dest_explorer: Mutex::new(FileExplorer::new()),
            key_manager: Mutex::new(KeyManager::new()),
            jobs: JobRegistry::default(),
            send_jobs: JobRegistry::default(),
            recv_jobs: ReceiverRegistry::default(),
            file_listener: Mutex::new(ListenerControl { stop: Arc::new(AtomicBool::new(false) )}),
            key_listener: Mutex::new(ListenerControl { stop: Arc::new(AtomicBool::new(false) )}),
            watcher: Arc::new(Mutex::new(None)),
            net_keys: Arc::new(Mutex::new(HashMap::new()))
        })
        .setup(|_app| {
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_files, change_dir, go_dir_back, set_current_dir,
            list_keys, find_keys_by_algo, generate_new_key, find_key, remove_key, load_keys, save_keys,
            encrypt_file, stop_processing, decrypt_file,
            stop_file_watching, start_file_watching,
            send_file, send_key, stop_sending,
            start_file_listening, stop_file_listening, approve_incoming, deny_incoming, stop_receiving,
            start_key_listening, stop_key_listening, get_network_keys, remove_network_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
