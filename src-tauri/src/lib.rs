// Only `lib.rs` and `commands/` may import `tauri`; `domain/` and `db/` must stay
// framework-free so they can be unit tested without a running app.
mod commands;
mod db;
mod domain;
mod error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
