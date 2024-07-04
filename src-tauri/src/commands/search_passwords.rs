use crate::{app_error::AppError, vault::PasswordEntry};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use uuid::Uuid;

#[tauri::command]
pub fn search_passwords(
    passwords: Vec<PasswordEntry>,
    search_term: String,
) -> Result<Vec<(i64, Uuid)>, AppError> {
    let matcher = &SkimMatcherV2::default().ignore_case();

    let result = passwords
        .iter()
        .map(|entry| {
            (
                matcher
                    .fuzzy_match(&entry.website(), &search_term)
                    .unwrap_or(0),
                entry.id(),
            )
        })
        .collect::<Vec<(i64, Uuid)>>();

    Ok(result)
}
