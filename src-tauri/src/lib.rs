mod files;

use std::sync::Mutex;
use crate::files::commands::{get_files, change_dir, go_dir_back};
use crate::files::file_explorer::FileExplorer;

struct AppState {
    source_explorer: Mutex<FileExplorer>,
    dest_explorer: Mutex<FileExplorer>,
}

#[allow(clippy::missing_panics_doc)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            source_explorer: Mutex::new(FileExplorer::new()),
            dest_explorer: Mutex::new(FileExplorer::new())
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
        .invoke_handler(tauri::generate_handler![get_files, change_dir, go_dir_back])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
