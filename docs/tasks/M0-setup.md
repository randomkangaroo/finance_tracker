# M0 – Setup

> Goal: an empty but complete project skeleton. Every layer from the React UI down to the SQLite file is wired up end-to-end, and every quality check (format, lint, types, tests, CI) runs and passes. No product features yet.
>
> Spec references: §2.2 (stack), §2.3 (architecture), §2.3.1 (backend rules), §5 (testing/CI).

## 0.1 Prerequisites

- [x] Install the Rust stable toolchain (`rustup`), with the `rustfmt` and `clippy` components
- [x] Install Node.js LTS and npm
- [x] Install the Tauri 2 Linux system dependencies (WebKitGTK 4.1, etc.) following the official Tauri prerequisites guide for your distro
- [x] **Done when:** `rustc --version`, `cargo clippy --version` and `node --version` all work

## 0.2 Scaffold the app

- [x] Generate the project with `npm create tauri-app@latest`, choosing **React + TypeScript** (Vite) and **npm**
- [x] Set the app identifier (e.g. `com.<you>.financetracker`), product name and window title in `tauri.conf.json`
- [x] Remove the template's demo code (greet command, logos, sample CSS)
- [x] Make sure `.gitignore` covers `node_modules/`, `dist/`, `src-tauri/target/` and local `.db` files
- [x] Commit `package-lock.json` and `src-tauri/Cargo.lock`
- [x] **Done when:** `npm run tauri dev` opens an empty window, and `npm run tauri build` produces a binary

## 0.3 Pin dependency versions

- [ ] Find a set of `tauri`, `tauri-specta` and `specta` versions that work together (check the `tauri-specta` README and release notes). Pin them exactly in `Cargo.toml` (`=x.y.z`)
- [ ] Add the Rust dependencies:
  - `rusqlite` with the **`bundled`** feature, so every machine uses the same SQLite version instead of the system one
  - `rusqlite_migration`
  - `time` (with the `serde`, `formatting` and `parsing` features)
  - `serde`, `thiserror`
- [ ] Add the frontend dependencies: `react-router`, `@tanstack/react-query`
- [ ] Record the pinned versions and why they were chosen in the README
- [ ] **Done when:** `cargo build` and `npm install` succeed with no version conflicts

## 0.4 Rust module layout

- [ ] Create this structure under `src-tauri/src/`:
  ```
  lib.rs          // Tauri builder setup: state, commands, bindings export
  main.rs         // calls lib::run()
  error.rs        // AppError
  commands/       // thin #[tauri::command] wrappers, one file per area
  core/           // domain types + business logic (no Tauri imports)
  db/
    mod.rs        // open(), open_in_memory(), pragmas
    migrations.rs // migration list
    sql/          // 001_*.sql, 002_*.sql, ...
  ```
- [ ] Rule (write it in the README): **`core/` and `db/` must never import `tauri`**. Only `commands/` and `lib.rs` may.
- [ ] **Done when:** the app compiles with the empty modules in place

## 0.5 Error type

- [ ] In `error.rs`, define `AppError` with `thiserror`. Starting variants: `Db(rusqlite::Error)`, `Migration(...)`, `NotFound { entity, id }`, `Validation(String)`, `Conflict(String)`
- [ ] Implement `Serialize` and `specta::Type` so the frontend gets a structured error (e.g. `{ kind: "Validation", message: "..." }`) rather than a bare string
- [ ] Add a `pub type AppResult<T> = Result<T, AppError>;` alias
- [ ] **Done when:** a unit test serializes each variant to the expected JSON shape

## 0.6 Database connection

- [ ] `db::open(path)`: create the parent directory if it's missing, open the connection, apply pragmas (`foreign_keys = ON`, `journal_mode = WAL`), then run migrations
- [ ] `db::open_in_memory()`: same pragmas and migrations, for tests
- [ ] In `lib.rs` setup: resolve `app_data_dir()/finance.db`, open it, and register `Mutex<Connection>` as managed state. Startup failures should show a clear error instead of a silent panic.
- [ ] **Done when:** tests confirm that `PRAGMA foreign_keys` returns `1` on both kinds of connection, and that the DB file appears in the app data dir after launch

## 0.7 Migrations

- [ ] Set up `rusqlite_migration` with SQL files loaded through `include_str!`
- [ ] `001_settings.sql`: create the `settings` table (`key TEXT PRIMARY KEY, value TEXT NOT NULL`). This proves the pipeline works and is needed in M1 anyway.
- [ ] Add a test that applies all migrations to an in-memory database and checks they're valid
- [ ] Document the rule: **never edit a migration once it's committed; add a new one**
- [ ] **Done when:** the migration test passes, and relaunching the app doesn't re-run migrations (`PRAGMA user_version` stays put)

## 0.8 `tauri-specta` bindings end-to-end

- [ ] Add one real command, `app_info`, returning `{ version, dbPath, schemaVersion }`. It should be `async`, per §2.3.1.
- [ ] Register commands through the `tauri-specta` builder, and export bindings to `src/bindings.ts`
- [ ] Export the bindings from a **Rust test** (or a small binary), not only on app launch. CI can then regenerate the file without opening a window.
- [ ] Commit `src/bindings.ts`. Exclude it from ESLint/Prettier.
- [ ] Show the result of `commands.appInfo()` on a temporary page in the frontend
- [ ] **Done when:** the window shows the version and DB path, and renaming a field in the Rust struct causes a TypeScript compile error after regenerating

## 0.9 Frontend skeleton

- [ ] Create this structure under `src/`:
  ```
  main.tsx        // providers: QueryClientProvider, RouterProvider
  bindings.ts     // generated, don't edit
  api/            // TanStack Query hooks wrapping commands + query keys
  routes/         // route components, one folder per screen
  components/     // shared UI components (each with Foo.tsx + Foo.module.css)
  lib/            // pure helpers (money, dates); no React
  styles/         // global.css: reset + CSS custom properties (colors, spacing, fonts)
  ```
- [ ] Set up React Router with an app layout (sidebar nav + content area) and placeholder routes: `/transactions`, `/accounts`, `/accounts/:id`, `/settings`. `/` redirects to `/transactions` until M3 adds the dashboard.
- [ ] Set up `QueryClient` with sensible defaults for a local app (e.g. no refetch on window focus, `retry: false`)
- [ ] Add a helper that turns `AppError` into a user-facing message, plus a simple error display component
- [ ] **Done when:** you can navigate between the placeholder pages and the `app_info` data loads through a query hook

## 0.10 Linting, formatting, type checking

- [ ] Rust: a `rustfmt.toml` (defaults are fine) and `cargo clippy --all-targets -- -D warnings` passing
- [ ] Frontend: ESLint (flat config) with `typescript-eslint` and `eslint-plugin-react-hooks`, plus Prettier
- [ ] Enable `"strict": true` in `tsconfig.json`
- [ ] npm scripts: `lint`, `format`, `format:check`, `typecheck` (`tsc --noEmit`), `test`
- [ ] **Done when:** every check passes on a clean checkout

## 0.11 Test setup

- [ ] Rust: one sample test each in `core/` and `db/` (beyond the ones above) to establish the pattern, e.g. a `test_conn()` helper that returns a migrated in-memory connection
- [ ] Frontend: Vitest with the `jsdom` environment, React Testing Library and `@testing-library/jest-dom`, plus a setup file
- [ ] Mock the Tauri IPC in tests with `mockIPC` from `@tauri-apps/api/mocks`
- [ ] Write one component test that renders the `app_info` page against a mocked `app_info` command
- [ ] **Done when:** `cargo test` and `npm test` both pass

## 0.12 CI (GitHub Actions)

- [ ] Workflow on push and pull request, Ubuntu runner, with the Tauri Linux dependencies installed, plus caching for Cargo and npm
- [ ] **Rust job:** `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`
- [ ] **Frontend job:** `npm ci`, `format:check`, `lint`, `typecheck`, `test`
- [ ] **Bindings check:** regenerate `src/bindings.ts`, then run `git diff --exit-code src/bindings.ts` to fail the build if someone forgot to commit updated bindings
- [ ] **Done when:** CI is green on `main`, and a deliberately broken formatting commit turns it red

## 0.13 Documentation

- [ ] README: prerequisites, dev commands (`npm run tauri dev`, tests, lint), where the database file lives, and how to regenerate bindings
- [ ] README: the architecture rules (layering, never editing old migrations, money as integer minor units)
- [ ] Link `SPEC.md` and `docs/tasks/`

## M0 exit criteria

- [ ] `npm run tauri dev` launches. The UI calls a typed Rust command, which reads from a migrated SQLite database.
- [ ] Format, lint, type checks and tests pass locally and in CI
- [ ] A new contributor (or future you) can get running from the README alone
