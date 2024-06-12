use crate::{app_error::AppError, vault::{PasswordEntry, VaultData}};

#[tauri::command]
pub fn update_password_entry(
    vault: String,
    password: String,
    entry_to_update: PasswordEntry,
) -> Result<(), AppError> {
    let mut vault_data = VaultData::load_from_file(vault, password.clone())?;

    let mut index: Option<usize> = None;

    for (i, entry) in vault_data.passwords.iter().enumerate() {
        if entry.id() == entry_to_update.id() {
            index = Some(i);
        }
    }

    if let Some(index) = index {
        vault_data.passwords[index] = entry_to_update;

        vault_data.save_to_file(password)?;

        Ok(())
    } else {
        Err(AppError::PasswordUpdatingFailed)
    }
}