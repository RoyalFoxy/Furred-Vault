// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use commands::{
    analyze_password, change_password_of_vault, close_splashscreen, create_password_entry,
    create_vault, delete_password_entry, delete_vault, get_uuid, get_vaults, load_vault,
    rename_vault, search_passwords, suggest_strong_password, update_password_entry,
};

pub mod app_error;
pub mod commands;
pub mod config;
pub mod consts;
pub mod encryption;
pub mod history;
pub mod serde_file;
pub mod serde_file_encryption;
pub mod util;
pub mod vault;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            analyze_password,
            change_password_of_vault,
            close_splashscreen,
            create_password_entry,
            create_vault,
            delete_password_entry,
            delete_vault,
            get_uuid,
            get_vaults,
            load_vault,
            rename_vault,
            search_passwords,
            suggest_strong_password,
            update_password_entry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
