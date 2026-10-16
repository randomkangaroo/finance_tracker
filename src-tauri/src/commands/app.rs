//! Commands about the app itself rather than the user's data.

use serde::Serialize;
use specta::Type;
use tauri::{AppHandle, State};

use super::{DbState, lock};
use crate::db;
use crate::error::AppResult;

#[derive(Debug, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub version: String,
    pub db_path: String,
    pub schema_version: u32,
}

#[tauri::command]
#[specta::specta]
pub async fn app_info(app: AppHandle, db: State<'_, DbState>) -> AppResult<AppInfo> {
    let conn = lock(&db);
    Ok(AppInfo {
        version: app.package_info().version.to_string(),
        db_path: conn.path().unwrap_or_default().to_owned(),
        schema_version: db::schema_version(&conn)?,
    })
}
