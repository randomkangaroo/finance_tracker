//! `AppError`, the single error type returned by every command.
//!
//! Across the IPC boundary every variant serializes to the same shape,
//! `{ kind: "Validation", message: "..." }`, so the frontend can switch on `kind`
//! and always has a human-readable `message` to show.

use serde::{Serialize, Serializer};
use specta::{Type, Types, datatype::DataType};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),

    #[error("database migration failed: {0}")]
    Migration(#[from] rusqlite_migration::Error),

    #[error("file system error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{entity} {id} not found")]
    NotFound { entity: &'static str, id: i64 },

    #[error("{0}")]
    Validation(String),

    #[error("{0}")]
    Conflict(String),
}

pub type AppResult<T> = Result<T, AppError>;

/// The variant name of an [`AppError`], exported to TypeScript as a string union.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Type)]
pub enum ErrorKind {
    Db,
    Migration,
    Io,
    NotFound,
    Validation,
    Conflict,
}

/// The wire format of an [`AppError`]. `AppError` itself holds library errors
/// that can't be serialized directly, so it converts to this on the way out.
#[derive(Serialize, Type)]
#[serde(rename = "AppError")]
struct SerializedAppError {
    kind: ErrorKind,
    message: String,
}

impl AppError {
    pub fn kind(&self) -> ErrorKind {
        match self {
            AppError::Db(_) => ErrorKind::Db,
            AppError::Migration(_) => ErrorKind::Migration,
            AppError::Io(_) => ErrorKind::Io,
            AppError::NotFound { .. } => ErrorKind::NotFound,
            AppError::Validation(_) => ErrorKind::Validation,
            AppError::Conflict(_) => ErrorKind::Conflict,
        }
    }
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        SerializedAppError {
            kind: self.kind(),
            message: self.to_string(),
        }
        .serialize(serializer)
    }
}

impl Type for AppError {
    fn definition(types: &mut Types) -> DataType {
        SerializedAppError::definition(types)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    fn to_json(err: AppError) -> Value {
        serde_json::to_value(err).unwrap()
    }

    #[test]
    fn db_error_serializes_with_kind_and_message() {
        assert_eq!(
            to_json(rusqlite::Error::QueryReturnedNoRows.into()),
            json!({ "kind": "Db", "message": "database error: Query returned no rows" })
        );
    }

    #[test]
    fn migration_error_serializes_with_kind_and_message() {
        let inner = rusqlite_migration::Error::InvalidUserVersion;
        let expected = format!("database migration failed: {inner}");
        assert_eq!(
            to_json(inner.into()),
            json!({ "kind": "Migration", "message": expected })
        );
    }

    #[test]
    fn io_error_serializes_with_kind_and_message() {
        let inner = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        assert_eq!(
            to_json(inner.into()),
            json!({ "kind": "Io", "message": "file system error: access denied" })
        );
    }

    #[test]
    fn not_found_serializes_with_kind_and_message() {
        let err = AppError::NotFound {
            entity: "account",
            id: 42,
        };
        assert_eq!(
            to_json(err),
            json!({ "kind": "NotFound", "message": "account 42 not found" })
        );
    }

    #[test]
    fn validation_serializes_with_kind_and_message() {
        let err = AppError::Validation("amount must not be zero".into());
        assert_eq!(
            to_json(err),
            json!({ "kind": "Validation", "message": "amount must not be zero" })
        );
    }

    #[test]
    fn conflict_serializes_with_kind_and_message() {
        let err = AppError::Conflict("account name already in use".into());
        assert_eq!(
            to_json(err),
            json!({ "kind": "Conflict", "message": "account name already in use" })
        );
    }
}
