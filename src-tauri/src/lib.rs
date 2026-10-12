// Only `lib.rs` and `commands/` may import `tauri`; `domain/` and `db/` must stay
// framework-free so they can be unit tested without a running app.
mod commands;
mod db;
mod domain;
mod error;

use std::sync::Mutex;

use tauri::Manager;

const DB_FILE_NAME: &str = "finance.db";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let result = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let path = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("could not find the app data directory: {e}"))?
                .join(DB_FILE_NAME);
            let conn = db::open(&path)
                .map_err(|e| format!("could not open the database at {}: {e}", path.display()))?;
            app.manage(Mutex::new(conn));
            Ok(())
        })
        .run(tauri::generate_context!());

    if let Err(e) = result {
        eprintln!("finance-tracker failed to start: {e}");
        std::process::exit(1);
    }
}
