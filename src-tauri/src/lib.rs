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
use tauri::Listener;
use crate::crypto::commands::{decrypt_file, encrypt_file, stop_processing};
use crate::files::commands::{change_dir, get_files, go_dir_back, set_current_dir};
use crate::files::commands::{start_file_watching, stop_file_watching};
use crate::key_manager::commands::{find_key, find_keys_by_algo, generate_new_key, list_keys};
use crate::network::commands::{send_file, send_key, start_file_listening, stop_file_listening, approve_incoming, deny_incoming,
                               start_key_listening, stop_key_listening, get_network_keys, remove_network_key, stop_sending};
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

#[allow(clippy::missing_panics_doc)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_files, change_dir, go_dir_back, set_current_dir,
            list_keys, find_keys_by_algo, generate_new_key, find_key,
            encrypt_file, stop_processing, decrypt_file,
            stop_file_watching, start_file_watching,
            send_file, send_key, stop_sending,
            start_file_listening, stop_file_listening, approve_incoming, deny_incoming,
            start_key_listening, stop_key_listening, get_network_keys, remove_network_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
