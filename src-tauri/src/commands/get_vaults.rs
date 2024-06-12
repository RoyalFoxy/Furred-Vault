use crate::{app_error::AppError, config::ApplicationConfig};

#[tauri::command]
pub fn get_vaults() -> Result<Vec<String>, AppError> {
    let app_config = ApplicationConfig::load()?;

    Ok(app_config.vaults)
}
