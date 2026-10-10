# M1 – Core Ledger

> Goal: a usable ledger. You can create accounts and categories, enter transactions by hand quickly, find them again with filters, and see correct balances.
>
> Spec references: §3 (data model), §4.1 (accounts), §4.2 (manual entry), §4.4 (categories), §5 (non-functional).
>
> Suggested order: backend sections 1.1–1.6 first (each fully tested), then frontend 1.7–1.12. Each numbered section is a reasonable size for one branch/PR.

## Backend

### 1.1 Schema migration

- [ ] `002_core_ledger.sql`:
  - `accounts`: `id`, `name` (NOT NULL), `type` (`CHECK` in the allowed list), `currency` (`CHECK length = 3`), `opening_balance INTEGER NOT NULL DEFAULT 0`, `archived INTEGER NOT NULL DEFAULT 0`, `created_at`, `updated_at`
  - `categories`: `id`, `name`, `kind` (`CHECK` in `income`/`expense`), `UNIQUE(name COLLATE NOCASE, kind)`. Leave out `parent_id`; subcategories are P1.
  - `transactions`: `id`, `account_id` (FK, `ON DELETE RESTRICT`), `date TEXT NOT NULL` (`CHECK` matching the `YYYY-MM-DD` pattern), `amount INTEGER NOT NULL`, `payee TEXT NOT NULL`, `category_id` (FK, nullable, `ON DELETE RESTRICT`), `notes`, `cleared INTEGER NOT NULL DEFAULT 0`, `created_at`, `updated_at`
  - Indexes: `transactions(account_id, date)`, `transactions(date)`, `transactions(category_id)`
  - Leave out tags, `transfer_id` and the import columns; later migrations add them (§2.3.1)
- [ ] `003_seed_defaults.sql`: insert the default categories from §4.4.1 and the `base_currency = 'USD'` setting. Seeding in a migration means it runs exactly once, so categories you delete stay deleted.
- [ ] Tests: the `CHECK` constraints and foreign keys reject bad data (invalid type, malformed date, an `account_id` that doesn't exist)
- [ ] **Done when:** migrations apply cleanly to a new database and to the M0 database

### 1.2 Domain types and validation (`core/`)

- [ ] Types (deriving `Serialize`, `Deserialize`, `specta::Type`): `Account`, `AccountType`, `NewAccount`, `AccountUpdate`; `Category`, `CategoryKind`, `NewCategory`; `Transaction`, `NewTransaction`, `TransactionUpdate`
- [ ] Dates use `time::Date` in Rust and serialize as `"YYYY-MM-DD"`. Check that the generated TS type is `string`.
- [ ] Validation functions (pure, unit-tested):
  - names are trimmed and non-empty, with a reasonable length cap
  - currency is 3 uppercase ASCII letters
  - payee is non-empty
  - amount is non-zero
- [ ] **Done when:** the validation unit tests cover the valid case and each rejection

### 1.3 Settings repository

- [ ] `get_setting(key)` and `set_setting(key, value)`, plus typed helpers for `base_currency` and `last_used_account_id`
- [ ] **Done when:** tests cover reading a default, overwriting a value, and reading a missing key

### 1.4 Accounts repository

- [ ] `create`, `update`, `get`, and `list(include_archived)`. List returns each account with its **balance** (`opening_balance + COALESCE(SUM(amount), 0)`) and **transaction count**, computed in a single query.
- [ ] `set_archived(id, bool)`
- [ ] `delete(id)` returns `Conflict` if the account has any transactions (§4.1)
- [ ] New accounts default to `base_currency` when no currency is given
- [ ] Tests: balance with no transactions, with mixed inflows and outflows, and with a negative opening balance (credit card); archived accounts hidden or shown; delete blocked when transactions exist and allowed when there are none
- [ ] **Done when:** all of the above pass

### 1.5 Categories repository

- [ ] `list` (grouped or sorted by kind, then name), `create`, `rename`
- [ ] `kind` can't be changed after creation (keeps reports consistent)
- [ ] `delete(id, reassign_to: Option<CategoryId>)`: inside one DB transaction, reassign the category's transactions (to another category of the same kind, or to uncategorized when `None`), then delete it. Reject reassigning to the category being deleted or to one of the other kind.
- [ ] Return a **usage count** per category so the UI can say "12 transactions will be moved"
- [ ] Tests: renaming to a duplicate name returns `Conflict`; reassignment moves every row; a failure partway through rolls back
- [ ] **Done when:** all of the above pass

### 1.6 Transactions repository

- [ ] `create`, `update`, `delete`, `get`, `set_cleared`
- [ ] Reject creating a transaction on an **archived** account, and reject category IDs that don't exist
- [ ] Creating a transaction updates `last_used_account_id`
- [ ] `list(filter, sort, page)` returns `{ items, totalCount }`:
  - **Filter:** `dateFrom`, `dateTo`, `accountIds`, `categoryIds` (including an "uncategorized" option), `minAmount`/`maxAmount` (compared against the absolute value), `search` (case-insensitive match on payee and notes, escaping `%` and `_` in `LIKE`)
  - **Sort:** by `date`, `amount` or `payee`, ascending or descending, always with `id` as a tie-breaker so paging is stable
  - **Page:** `limit` and `offset`
  - Build the SQL with bound parameters only, never string-concatenated values
  - Each item includes the account name and category name, so the UI doesn't need extra lookups
- [ ] Tests: each filter alone and combined; sort order and tie-breaking; pagination boundaries; search with special characters
- [ ] **Performance:** a test or dev script that inserts 50,000 transactions and checks that one page loads well under 500 ms (§5). Inspect the query plan with `EXPLAIN QUERY PLAN` if it doesn't.
- [ ] **Done when:** all of the above pass

### 1.7 Commands and bindings

- [ ] Add thin `async` commands for everything in 1.3–1.6 (`list_accounts`, `create_account`, …, `list_transactions`). Each one locks the connection, calls the repository, and returns `AppResult<T>`.
- [ ] Register them with `tauri-specta`, regenerate `bindings.ts`, and commit it
- [ ] Remove the temporary `app_info` page, or move its info to Settings → About
- [ ] **Done when:** CI is green, including the bindings check

### 1.8 Dev sample data

- [ ] A debug-only way to fill a database with realistic fake data (a few accounts, about a year of transactions). This could be a debug-only command behind a Settings button, or a small `cargo run --bin seed` binary.
- [ ] **Done when:** one action produces a database you can explore the UI with

## Frontend

### 1.9 Money and date helpers (`lib/`)

- [ ] `formatMoney(minor, currency)`: uses `Intl.NumberFormat`, with the number of decimal places taken from the currency (2 for USD)
- [ ] `parseAmountInput(text, currency)`: parses strings such as `"12"`, `"12.5"`, `"1,234.56"` and `"$12.00"` into **integer minor units by string manipulation, never float multiplication**. Rejects too many decimal places, negative signs (the toggle sets the sign), empty input and garbage.
- [ ] `todayLocal()`: returns the **local** date as `YYYY-MM-DD`. Don't use `toISOString()`, which converts to UTC and returns the wrong day in the evening in US time zones.
- [ ] Thorough Vitest unit tests for all three
- [ ] **Done when:** the tests pass

### 1.10 API layer (`api/`)

- [ ] Query key factory (e.g. `keys.accounts.list(...)`, `keys.transactions.list(filter, sort, page)`)
- [ ] Query and mutation hooks for each command
- [ ] Invalidation rules:
  - **transaction** mutations invalidate transaction lists **and** account lists (balances change)
  - category mutations invalidate categories and transactions
  - account mutations invalidate accounts and transactions (names shown in rows)
- [ ] Mutations surface `AppError` messages through the shared error helper
- [ ] **Done when:** a hook test with mocked IPC confirms that creating a transaction refetches account balances

### 1.11 Accounts screens

- [ ] `/accounts`: table of accounts showing name, type, balance and transaction count, with a "Show archived" toggle and a total balance at the bottom
- [ ] Create/edit account form: name, type, currency (defaults to base currency), opening balance (using `parseAmountInput`; a negative opening balance is allowed here for credit cards and loans)
- [ ] Archive/unarchive actions. Delete is only enabled when the account has no transactions, and asks for confirmation.
- [ ] `/accounts/:id`: account header with its balance, plus the transaction table (1.12) pre-filtered to that account
- [ ] Component tests: form validation messages; delete disabled when the account has transactions
- [ ] **Done when:** the full account lifecycle works in the app

### 1.12 Transactions screen

- [ ] **Quick-add form** at the top of `/transactions`:
  - Fields in tab order: date (defaults to today), account (defaults to last used), Expense/Income toggle (defaults to Expense), amount, payee, category, notes
  - The category dropdown shows categories matching the toggle's kind, plus "Uncategorized"
  - Enter submits. After saving, keep the date and account, clear the other fields, and move focus back to the amount field so you can enter the next one right away.
  - Shows inline validation errors
  - **Target:** entering a transaction takes under 10 seconds using only the keyboard (§1.2)
- [ ] **Transaction table:**
  - Columns: date, account, payee, category, amount (expenses and income visually distinct, not by color alone), cleared checkbox
  - Clicking a sortable column header changes the sort
  - Pagination controls showing "Showing 1–50 of 1,234"
  - Edit a row (inline, or in a form/dialog that reuses the quick-add fields), with Esc to cancel
  - Delete with confirmation
- [ ] **Filter bar:** date range, account(s), category(ies) including uncategorized, amount range, search text (debounced), and a "Clear filters" button
- [ ] **Store filter, sort and page state in the URL search params** (React Router), so filtered views survive navigation and back/forward works
- [ ] Empty states ("No transactions yet", "No matches for these filters") and loading states
- [ ] Component tests: the toggle produces a negative amount for expenses; focus returns to amount after submit; filters update the URL and the query
- [ ] **Done when:** you can enter, find, edit and delete transactions entirely from this screen

### 1.13 Settings screen

- [ ] **Categories section:** income and expense lists showing usage counts; add, rename, and delete. Deleting opens a dialog: "Move N transactions to: [category of same kind / Uncategorized]".
- [ ] **General section:** base currency (affects the default for new accounts only)
- [ ] **About section:** app version and database file path
- [ ] **Done when:** categories can be fully managed, and deleting one never leaves transactions pointing at a missing category

### 1.14 Accessibility and polish pass

- [ ] Every input has a label, focus outlines are visible, dialogs trap focus and close on Esc
- [ ] Check color contrast of the CSS custom properties (WCAG AA)
- [ ] Check the layout at typical desktop window sizes (down to about 1024px wide)

## M1 exit criteria

- [ ] Create a checking account and a credit card, enter 20+ transactions by keyboard (each under 10 s), and the balances match a hand calculation
- [ ] Filtering, sorting and paging work together on the 50k-row sample database, staying responsive (< 500 ms per page)
- [ ] Deleting a category safely reassigns its transactions; deleting an account with transactions is blocked
- [ ] All Rust and frontend tests pass, and CI is green
- [ ] Data survives restarting the app
