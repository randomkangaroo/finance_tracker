//! The migration list. SQL files live in `sql/` and are loaded with `include_str!`.
//! Never edit a migration once it is committed; add a new one.

use rusqlite_migration::{M, Migrations};

const MIGRATION_LIST: &[M<'static>] = &[M::up(include_str!("sql/001_settings.sql"))];

pub const MIGRATIONS: Migrations<'static> = Migrations::from_slice(MIGRATION_LIST);

/// The schema version a fully migrated database reports in `PRAGMA user_version`.
#[cfg(test)]
pub const LATEST_VERSION: i64 = MIGRATION_LIST.len() as i64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_are_valid() {
        // Applies every migration to a fresh in-memory database and checks
        // the SQL runs and the resulting schema passes foreign key checks.
        MIGRATIONS.validate().unwrap();
    }
}
