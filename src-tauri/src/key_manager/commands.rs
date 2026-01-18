use crate::AppState;
use crate::key_manager::errors::KeysError;
use crate::key_manager::key::{PlainKey, ShortKey};
use crate::key_manager::key_sizes::KeySizes;

#[tauri::command]
pub async fn list_keys(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<ShortKey>, KeysError> {
    let key_manager = state.key_manager.lock()
        .map_err(|e| KeysError::KeyManagerInternalError(e.to_string()))?;

    key_manager.list_keys()
}

#[tauri::command]
pub async fn find_keys_by_algo(
    state: tauri::State<'_, AppState>,
    algorithm: String,
    mode: Option<String>,
) -> Result<Vec<String>, KeysError> {
    let key_manager = state.key_manager.lock()
        .map_err(|e| KeysError::KeyManagerInternalError(e.to_string()))?;

    let (key_size, iv_size) = KeySizes::get_size_of(&algorithm, &mode)?;

    key_manager.find_keys_by_size(key_size, iv_size)
}

#[tauri::command]
pub async fn generate_new_key(
    state: tauri::State<'_, AppState>,
    algorithm: String,
    mode: Option<String>,
    name: String,
    password: String
) -> Result<PlainKey, KeysError> {
    let mut key_manager = state.key_manager.lock()
        .map_err(|e| KeysError::KeyManagerInternalError(e.to_string()))?;

    let (key_size, iv_size) = KeySizes::get_size_of(&algorithm, &mode)?;
    
    key_manager.generate_new(name, password, key_size, iv_size)
}

#[tauri::command]
pub async fn find_key(
    state: tauri::State<'_, AppState>,
    name: String,
    password: String
) -> Result<PlainKey, KeysError> {
    let key_manager = state.key_manager.lock()
        .map_err(|e| KeysError::KeyManagerInternalError(e.to_string()))?;

    key_manager.find_key(name, password)
}

#[tauri::command]
pub async fn remove_key(
    state: tauri::State<'_, AppState>,
    name: String,
) -> Result<(), KeysError> {
    let mut key_manager = state.key_manager.lock()
        .map_err(|e| KeysError::KeyManagerInternalError(e.to_string()))?;

    key_manager.delete_key(name)
}

#[tauri::command]
    pub async fn load_keys(
    state: tauri::State<'_, AppState>,
    filename: String
) -> Result<(), KeysError> {
    let key_explorer = state.source_explorer.lock()
        .map_err(|e| KeysError::KeyManagerInternalError(e.to_string()))?;

    let mut key_manager = state.key_manager.lock()
        .map_err(|e| KeysError::KeyManagerInternalError(e.to_string()))?;

    let mut key_dest = key_explorer.get_current_path_buf();
    key_dest.push(filename);

    key_manager.load_from_disk(&key_dest)
}

#[tauri::command]
pub async fn save_keys(
    state: tauri::State<'_, AppState>,
    filename: String
) -> Result<(), KeysError> {
    let key_explorer = state.source_explorer.lock()
        .map_err(|e| KeysError::KeyManagerInternalError(e.to_string()))?;

    let key_manager = state.key_manager.lock()
        .map_err(|e| KeysError::KeyManagerInternalError(e.to_string()))?;

    let mut key_dest = key_explorer.get_current_path_buf();
    key_dest.push(format!("{filename}.keys"));

    key_manager.save_to_disk(&key_dest)
}