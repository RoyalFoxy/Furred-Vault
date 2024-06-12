use crate::app_error::AppError;
use passwords::PasswordGenerator;

#[tauri::command]
pub fn suggest_strong_password() -> Result<String, AppError> {
    let pg = PasswordGenerator {
        length: 16,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: true,
        exclude_similar_characters: false,
        strict: true,
    };

    let password = pg.generate_one().map_err(|e| AppError::String(e.to_owned()))?;

    Ok(password)
}
