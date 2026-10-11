# Finance Tracker

An app to help track personal finances

## Architecture rules

The Rust backend in `src-tauri/src/` is layered:

| Module      | Contents                                                       |
| ----------- | -------------------------------------------------------------- |
| `lib.rs`    | Tauri builder setup: managed state, commands, bindings export  |
| `main.rs`   | Calls `finance_tracker_lib::run()`                             |
| `error.rs`  | `AppError`, the error type every command returns               |
| `commands/` | Thin `#[tauri::command]` wrappers, one file per area           |
| `domain/`   | Domain types and business logic                                |
| `db/`       | Connection setup, pragmas, migrations (`db/sql/NNN_*.sql`)     |

- **`domain/` and `db/` must never import `tauri`.** Only `commands/` and `lib.rs` may. This
  keeps the business logic and database code testable with plain `cargo test`.

## Pinned dependency versions

Most dependencies use normal semver ranges and are held in place by `Cargo.lock` and
`package-lock.json`. The ones below are pinned exactly (`=x.y.z` in `src-tauri/Cargo.toml`)
because they have to move together.

| Crate               | Version       | Why                                                                                                     |
| ------------------- | ------------- | ------------------------------------------------------------------------------------------------------- |
| `tauri`             | `2.12.3`      | `tauri-specta` hooks into Tauri's command/invoke internals, so the pair must be known to work together. |
| `tauri-build`       | `2.7.2`       | Build-time companion to `tauri`; pinned so it can't drift ahead of the `tauri` version.                 |
| `specta`            | `2.0.0-rc.25` | This is the newest current version of `specta`.                                                         |
| `specta-typescript` | `0.0.12`      | The TypeScript exporter for `specta`; this is the version `tauri-specta 2.0.0-rc.25` is built against.  |
| `tauri-specta`      | `2.0.0-rc.25` | Release candidate; must match `specta` exactly.                                                         |

`tauri-specta`, `specta` and `specta-typescript` versions come from the same `tauri-specta`
release. The frontend `@tauri-apps/api` (2.12.x) is kept on the same minor version as the
`tauri` crate, as Tauri expects.

Other notable choices (ranges, not exact pins):

- `rusqlite` `0.40` with the `bundled` feature, so every machine uses the same compiled-in
  SQLite instead of the system library. `rusqlite_migration` `2.6` supports this `rusqlite`.
- `time` `0.3` with `serde`, `formatting` and `parsing`.
- Frontend: `react` 19, `react-router` 8, `@tanstack/react-query` 5.

**Bumping the pinned set:** check the `tauri-specta` release notes for the `specta`,
`specta-typescript` and `tauri` versions it targets, update all of them together, then run
`cargo build` and regenerate the bindings.
