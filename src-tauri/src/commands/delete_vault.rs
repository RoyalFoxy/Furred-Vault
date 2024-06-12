use crate::{
    app_error::AppError, config::ApplicationConfig, util::get_file_path, vault::VaultData,
};
use std::fs;

#[tauri::command]
pub fn delete_vault(vault: String) -> Result<(), AppError> {
    let mut app_config = ApplicationConfig::load()?;

    if !app_config.vaults.contains(&vault) {
        return Err(AppError::VaultDoesNotExist);
    }

    let mut index: Option<usize> = None;

    for (i, entry) in app_config.vaults.iter().enumerate() {
        if entry == &vault {
            index = Some(i);
        }
    }

    if let Some(index) = index {
        app_config.vaults.remove(index);
    } else {
        return Err(AppError::InternalServerError);
    }

    app_config.save()?;

    let path_of_data = get_file_path(VaultData::file_name(vault))?;

    fs::remove_file(path_of_data)?;

    Ok(())
}
