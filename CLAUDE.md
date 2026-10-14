# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

A single-user, offline desktop personal finance tracker: Tauri 2 + Rust backend + SQLite (`rusqlite`), with a React + TypeScript (Vite) frontend. It is also a learning project, so clean layering and test coverage come before shortcuts.

- `SPEC.md` is the source of truth for requirements, the data model, and decisions. Items marked **[DECIDE]** are still open; don't settle them without asking.
- `docs/tasks/M*.md` hold milestone checklists. Work goes section by section (e.g. "M0 0.7"), and finished items get checked off there. M0 (setup) is in progress; M1 (core ledger) is next.

## Commands

The Rust crate lives in `src-tauri/`, so run cargo commands from there (or pass `--manifest-path src-tauri/Cargo.toml`).

```sh
npm run tauri dev                          # run the app (starts Vite on :1420 + the Rust backend)
npm run tauri build                        # release bundle
npm run build                              # frontend only: tsc + vite build

cd src-tauri
cargo test                                 # all Rust tests
cargo test db::tests::                     # tests in one module
cargo test file_connection_enforces        # a single test, matched by name substring
cargo fmt --check
cargo clippy --all-targets -- -D warnings  # the lint bar CI will enforce
```

These aren't set up yet (planned in M0 0.10–0.12): ESLint/Prettier, the npm `lint`/`format`/`typecheck`/`test` scripts, Vitest, and GitHub Actions CI.

## Architecture

The data path is: React UI → `tauri-specta` typed command wrappers → `commands/` → `domain/` + `db/` → SQLite file. Rust owns the database and all business logic, and the frontend never writes SQL (`tauri-plugin-sql` isn't used).

Backend layering in `src-tauri/src/`:

- **`domain/` and `db/` must never import `tauri`.** Only `lib.rs` and `commands/` may. This keeps the core testable with plain `cargo test`.
- `commands/`: thin `#[tauri::command]` wrappers, one file per area. They validate input, lock the connection, call into domain/db, and return `AppResult<T>`. Commands must be `async` (or `#[tauri::command(async)]`). `rusqlite` is blocking, and non-async commands run on the UI thread.
- `lib.rs`: in `setup`, it opens `app_data_dir()/finance.db` and registers a single `Mutex<rusqlite::Connection>` as managed state (no pool). Commands and `tauri-specta` binding export get registered here too.
- `error.rs`: `AppError` (built with `thiserror`) is the one error type for every command. It serializes across IPC as `{ kind, message }`, where `kind` is the `ErrorKind` string union. If you add a variant, update `AppError::kind()`, `ErrorKind`, and the serialization tests.
- `db/mod.rs`: `open(path)` and the test-only `open_in_memory()` both go through `init()`. That sets `foreign_keys = ON` (it has to come before migrating), sets `journal_mode = WAL`, then runs migrations. Tests should use `open_in_memory()`.
- `db/migrations.rs` and `db/sql/NNN_*.sql`: migrations are loaded with `include_str!` and run by `rusqlite_migration`, which tracks progress in `PRAGMA user_version`. **Never edit a committed migration; add a new numbered file.** Each milestone adds only the schema it needs. Seed data (like default categories) also goes in migrations, so it runs exactly once.

Planned but not built yet (M0 0.8–0.9): Rust→TS bindings exported to `src/bindings.ts`. That file is generated, so don't edit it by hand. It should be regenerable from a Rust test, and CI will check it's up to date. The frontend layout will be `api/` (TanStack Query hooks + query keys), `routes/`, `components/` (`Foo.tsx` + `Foo.module.css`), `lib/` (pure helpers, no React), and `styles/`.

## Domain rules

- Money is always integer minor units (`i64` in Rust, a plain `number` in TS). Never use floats, including when parsing user input. Format as a decimal only at render time.
- Amounts are signed: negative means outflow. The UI takes a positive amount plus an Expense/Income toggle.
- Every amount is tied to a currency through its account, even though v1 uses a single currency.
- Balances are derived (`opening_balance + SUM(amount)`) and never stored or edited directly.
- Dates are stored as `TEXT`: `YYYY-MM-DD` for transaction dates (no time zone), UTC RFC 3339 for `created_at`/`updated_at`. On the Rust side they're `time::Date`. In the frontend, get "today" from the local date, not `toISOString()`.
- Multi-step writes run inside a DB transaction. Build SQL with bound parameters only.

## Dependency pins

`tauri`, `tauri-build`, `specta`, `specta-typescript`, and `tauri-specta` are pinned exactly (`=x.y.z`) in `src-tauri/Cargo.toml` because they have to move together. Don't bump one on its own. Follow the procedure in the README ("Bumping the pinned set"). `@tauri-apps/api` stays on the same minor version as the `tauri` crate.
