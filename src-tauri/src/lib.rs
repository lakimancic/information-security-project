mod files;
pub mod crypto;
mod key_manager;

use std::sync::Mutex;
use crate::files::commands::{get_files, change_dir, go_dir_back, set_current_dir};
use crate::key_manager::commands::{list_keys, find_keys_by_algo, generate_new_key, find_key};
use crate::files::file_explorer::FileExplorer;
use crate::key_manager::key_manager::KeyManager;

struct AppState {
    source_explorer: Mutex<FileExplorer>,
    dest_explorer: Mutex<FileExplorer>,
    key_manager: Mutex<KeyManager>,
}

#[allow(clippy::missing_panics_doc)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            source_explorer: Mutex::new(FileExplorer::new()),
            dest_explorer: Mutex::new(FileExplorer::new()),
            key_manager: Mutex::new(KeyManager::new())
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
            list_keys, find_keys_by_algo, generate_new_key, find_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
