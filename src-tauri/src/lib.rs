// Only `lib.rs` and `commands/` may import `tauri`; `domain/` and `db/` must stay
// framework-free so they can be unit tested without a running app.
mod commands;
mod db;
mod domain;
mod error;

use std::sync::Mutex;

use specta_typescript::Typescript;
use tauri::Manager;
use tauri_specta::{Builder, ErrorHandlingMode, collect_commands};

const DB_FILE_NAME: &str = "finance.db";

/// Where the generated TypeScript bindings are written.
const BINDINGS_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../src/bindings.ts");

/// Every command callable from the frontend. Register new commands here.
fn specta_builder() -> Builder<tauri::Wry> {
    Builder::new()
        .commands(collect_commands![commands::app::app_info])
        // Failed commands reject their promise with an `AppError`, which is
        // what TanStack Query expects from a query function.
        .error_handling(ErrorHandlingMode::Throw)
}

fn export_bindings(builder: &Builder<tauri::Wry>) {
    builder
        .export(Typescript::default(), BINDINGS_PATH)
        .expect("failed to export TypeScript bindings");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();

    // Keep the bindings in sync while developing. CI regenerates them with the
    // `export_bindings` test instead.
    #[cfg(debug_assertions)]
    export_bindings(&builder);

    let result = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(builder.invoke_handler())
        .setup(|app| {
            let path = app
                .path()
                .app_data_dir()
                .map_err(|e| format!("could not find the app data directory: {e}"))?
                .join(DB_FILE_NAME);
            let conn = db::open(&path)
                .map_err(|e| format!("could not open the database at {}: {e}", path.display()))?;
            app.manage::<commands::DbState>(Mutex::new(conn));
            Ok(())
        })
        .run(tauri::generate_context!());

    if let Err(e) = result {
        eprintln!("finance-tracker failed to start: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Regenerates `src/bindings.ts` without launching the app:
    /// `cargo test export_bindings`.
    #[test]
    fn export_bindings() {
        super::export_bindings(&specta_builder());
    }
}
