#[tauri::command]
pub fn hello() -> String{
  "hello".to_string()
}

#[tauri::command]
pub fn is_app() -> bool{
  true
}