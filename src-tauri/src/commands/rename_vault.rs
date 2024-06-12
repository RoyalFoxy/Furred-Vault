use crate::{
    app_error::AppError, config::ApplicationConfig, util::get_file_path, vault::VaultData,
};
use std::fs;

#[tauri::command]
pub fn rename_vault(
    old_vault: String,
    new_vault: String,
    password: String,
) -> Result<(), AppError> {
    let mut app_config = ApplicationConfig::load()?;

    if !app_config.vaults.contains(&old_vault) {
        return Err(AppError::VaultDoesNotExist);
    }

    if app_config.vaults.contains(&new_vault) {
        return Err(AppError::VaultAlreadyExists);
    }

    let mut index: Option<usize> = None;

    for (i, entry) in app_config.vaults.iter().enumerate() {
        if entry == &old_vault {
            index = Some(i);
        }
    }

    if let Some(index) = index {
        app_config.vaults[index] = new_vault.clone();
    } else {
        return Err(AppError::InternalServerError);
    }

    app_config.save()?;

    let mut data = VaultData::load_from_file(old_vault.clone(), password.clone())?;

    let path_of_old_data = get_file_path(VaultData::file_name(&old_vault))?;
    let path_of_old_temp_data = get_file_path(VaultData::file_name(format!("{old_vault}-temp")))?;

    fs::rename(path_of_old_data, &path_of_old_temp_data)?;

    data.rename(new_vault);
    data.save_to_file(password)?;

    fs::remove_file(path_of_old_temp_data)?;

    Ok(())
}
