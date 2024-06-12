use crate::{
    app_error::AppError,
    vault::{PasswordEntry, VaultData},
};

#[tauri::command]
pub fn create_password_entry(
    vault: String,
    password: String,
    entry: PasswordEntry,
) -> Result<(), AppError> {
    let mut vault_data = VaultData::load_from_file(vault, password.clone())?;

    vault_data.passwords.push(entry);

    vault_data.save_to_file(password)?;

    Ok(())
}
