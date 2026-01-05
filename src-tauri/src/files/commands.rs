use crate::AppState;
use crate::files::errors::FilesError;
use crate::files::file_entry::{FileEntry, FileExplore};

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
) -> Result<(), FilesError> {
    let mut explorer = if source {
        state.source_explorer.lock()
    } else {
        state.dest_explorer.lock()
    }.map_err(|_| FilesError::ExplorerInternalError)?;

    explorer.change_dir(new_dir);
    Ok(())
}

#[tauri::command]
pub async fn go_dir_back(
    state: tauri::State<'_, AppState>,
    source: bool
) -> Result<(), FilesError> {
    let mut explorer = if source {
        state.source_explorer.lock()
    } else {
        state.dest_explorer.lock()
    }.map_err(|_| FilesError::ExplorerInternalError)?;
    explorer.go_back();

    Ok(())
}