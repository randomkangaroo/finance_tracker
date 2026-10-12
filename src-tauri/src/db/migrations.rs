//! The migration list. SQL files live in `sql/` and are loaded with `include_str!`.
//! Never edit a migration once it is committed; add a new one.

use rusqlite_migration::{M, Migrations};

const MIGRATION_LIST: &[M<'static>] = &[M::up(include_str!("sql/001_settings.sql"))];

pub const MIGRATIONS: Migrations<'static> = Migrations::from_slice(MIGRATION_LIST);
