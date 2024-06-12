use crate::{app_error::AppError, vault::VaultData};

#[tauri::command]
pub fn load_vault(vault: String, password: String) -> Result<VaultData, AppError> {
    VaultData::load_from_file(vault, password)
}
