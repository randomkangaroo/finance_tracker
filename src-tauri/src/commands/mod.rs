//! Thin `#[tauri::command]` wrappers, one file per area. Each one validates its
//! input, calls into `domain`/`db`, and maps the result to an `AppResult`.
