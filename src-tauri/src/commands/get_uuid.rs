use uuid::Uuid;

#[tauri::command]
pub fn get_uuid() -> Uuid {
    Uuid::new_v4()
}
