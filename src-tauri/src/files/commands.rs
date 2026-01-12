use std::sync::atomic::Ordering;
use crate::AppState;
use crate::crypto::errors::CryptoError;
use crate::files::errors::FilesError;
use crate::files::file_entry::{FileEntry, FileExplore};
use crate::files::fsw::{start_watcher, WatchMode};

#[tauri::command]
pub async fn get_files(
    state: tauri::State<'_, AppState>,
    source: bool
) -> Result<FileExplore, FilesError> {
    let mut explorer = if source {
        state.source_explorer.lock()
    } else {
        state.dest_explorer.lock()
    }.map_err(|_| FilesError::ExplorerInternalError)?;
    let pwd = explorer.get_current_path();

    Ok(FileExplore {
        files: explorer.list_entries()?,
        pwd,
    })
}

#[tauri::command]
pub async fn change_dir(
    state: tauri::State<'_, AppState>,
    new_dir: String,
    source: bool
) -> Result<bool, FilesError> {
    let mut explorer = if source {
        state.source_explorer.lock()
    } else {
        state.dest_explorer.lock()
    }.map_err(|_| FilesError::ExplorerInternalError)?;

    Ok(explorer.change_dir(new_dir))
}

#[tauri::command]
pub async fn go_dir_back(
    state: tauri::State<'_, AppState>,
    source: bool
) -> Result<bool, FilesError> {
    let mut explorer = if source {
        state.source_explorer.lock()
    } else {
        state.dest_explorer.lock()
    }.map_err(|_| FilesError::ExplorerInternalError)?;

    Ok(explorer.go_back())
}

#[tauri::command]
pub async fn set_current_dir(
    state: tauri::State<'_, AppState>,
    new_dir: String,
    source: bool
) -> Result<bool, FilesError> {
    let mut explorer = if source {
        state.source_explorer.lock()
    } else {
        state.dest_explorer.lock()
    }.map_err(|_| FilesError::ExplorerInternalError)?;

    Ok(explorer.set_current_path(new_dir))
}

#[tauri::command]
pub fn start_file_watching(
    state: tauri::State<AppState>,
    app: tauri::AppHandle,
    mode: WatchMode,
) -> Result<(), FilesError> {
    let mut watcher = state.watcher.lock().unwrap();

    if watcher.is_some() {
        return Err(FilesError::WatcherAlreadyRunning);
    }

    let jobs = state.jobs.clone();

    let source_explorer = state.source_explorer.lock()
        .map_err(|_| FilesError::WatcherInternalError)?;

    let destination_explorer = state.dest_explorer.lock()
        .map_err(|_| FilesError::WatcherInternalError)?;

    let source_path = source_explorer.get_current_path_buf();
    let destination_path = destination_explorer.get_current_path_buf();

    let service = start_watcher(source_path, destination_path, app, jobs, mode);
    *watcher = Some(service);

    Ok(())
}

#[tauri::command]
pub fn stop_file_watching(state: tauri::State<AppState>) {
    if let Some(service) = state.watcher.lock().unwrap().take() {
        service.stop.store(true, Ordering::SeqCst);
        let _ = service.handle.join();
    }
}