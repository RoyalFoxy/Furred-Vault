use crate::{app_error::AppError, config::ApplicationConfig, vault::VaultData};

#[tauri::command]
pub fn create_vault(vault: String, password: String) -> Result<(), AppError> {
    let mut app_config = ApplicationConfig::load()?;

    if app_config
        .vaults
        .iter()
        .map(|vault| vault.to_ascii_lowercase())
        .collect::<Vec<String>>()
        .contains(&vault.to_ascii_lowercase())
    {
        return Err(AppError::VaultAlreadyExists);
    }

    app_config.vaults.push(vault.clone());
    app_config.save()?;

    VaultData::new(vault.clone()).save_to_file(password)?;

    Ok(())
}
