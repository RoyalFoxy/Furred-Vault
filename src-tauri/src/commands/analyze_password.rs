use passwords::{analyzer, scorer};

#[tauri::command]
pub fn analyze_password(password: String) -> f64 {
    scorer::score(&analyzer::analyze(password))
}
