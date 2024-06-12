use crate::{app_error::AppError, vault::VaultData};

#[tauri::command]
pub fn change_password_of_vault(
    vault: String,
    old_password: String,
    new_password: String,
) -> Result<(), AppError> {
    let data = VaultData::load_from_file(vault, old_password)?;
    data.save_to_file(new_password)?;

    Ok(())
}
