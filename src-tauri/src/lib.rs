#![feature(mpmc_channel)]

mod files;
pub mod crypto;
mod key_manager;
mod network;
mod progress;

use std::sync::{Arc, Mutex};
use crate::crypto::api::jobs::JobRegistry;
use crate::crypto::commands::{decrypt_file, encrypt_file, stop_processing};
use crate::files::commands::{change_dir, get_files, go_dir_back, set_current_dir};
use crate::files::commands::{start_file_watching, stop_file_watching};
use crate::key_manager::commands::{find_key, find_keys_by_algo, generate_new_key, list_keys};
use crate::files::file_explorer::FileExplorer;
use crate::files::watch::WatcherState;
use crate::key_manager::key_manager::KeyManager;

pub struct AppState {
    jobs: JobRegistry,
    source_explorer: Mutex<FileExplorer>,
    dest_explorer: Mutex<FileExplorer>,
    key_manager: Mutex<KeyManager>,
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
            watcher: Arc::new(Mutex::new(None)),
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
