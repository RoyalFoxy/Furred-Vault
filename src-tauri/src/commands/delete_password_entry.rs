use crate::{app_error::AppError, vault::VaultData};
use uuid::Uuid;

#[tauri::command]
pub fn delete_password_entry(
    vault: String,
    password: String,
    entry_id_to_delete: Uuid,
) -> Result<(), AppError> {
    let mut vault_data = VaultData::load_from_file(vault, password.clone())?;

    let mut index: Option<usize> = None;

    for (i, entry) in vault_data.passwords.iter().enumerate() {
        if entry.id() == entry_id_to_delete {
            index = Some(i);
        }
    }

    if let Some(index) = index {
        vault_data.passwords.remove(index);

        vault_data.save_to_file(password)?;

        Ok(())
    } else {
        Err(AppError::PasswordDeletionFailed)
    }
}
