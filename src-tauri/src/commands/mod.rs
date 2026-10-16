//! Thin `#[tauri::command]` wrappers, one file per area. Each one validates its
//! input, calls into `domain`/`db`, and maps the result to an `AppResult`.

pub mod app;

use std::sync::{Mutex, MutexGuard, PoisonError};

use rusqlite::Connection;

/// The database connection, registered as Tauri managed state in `lib.rs`.
pub type DbState = Mutex<Connection>;

/// Locks the connection. A panic while the lock was held poisons the mutex,
/// but the connection is still usable (any open transaction was rolled back
/// when it was dropped), so recover it rather than failing every later command.
fn lock(db: &DbState) -> MutexGuard<'_, Connection> {
    db.lock().unwrap_or_else(PoisonError::into_inner)
}
