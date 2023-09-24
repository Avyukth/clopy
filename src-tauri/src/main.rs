// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_clipboard_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn save_clipboard_data(data: String) {
    // Implement the logic to save clipboard data
    // This is just a placeholder
    println!("Clipboard Data: {}", data);
}
