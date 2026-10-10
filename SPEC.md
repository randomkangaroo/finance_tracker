# Personal Finance Tracker — Specification (Draft)

> Status: **Draft v0.2** — items marked **[DECIDE]** are open questions to resolve before or during implementation.

## 1. Overview

### 1.1 Purpose
A personal desktop application for recording income and expenses, organizing them into categories, tracking budgets, and visualizing where money goes over time.

### 1.2 Goals
- Make it fast to record a transaction by hand (< 10 seconds).
- Make it easy to bring in transactions from bank-exported CSV files.
- Give a clear answer to "How much did I spend on X this month?" and "Am I on budget?"
- Keep all financial data on the local machine, under the user's control.
- Serve as a learning project: favor clean architecture, clear separation of concerns, and good test coverage over shortcuts.

### 1.3 Non-Goals
- **Bank sync / account aggregation** (Plaid, SimpleFIN, etc.) — permanently out of scope, not a future goal.
- Multi-user support, user accounts, or authentication
- Hosting, cloud sync, or any required network services
- Investment portfolio tracking / stock prices
- Tax preparation
- Bill payment or moving money

### 1.4 Target User
A single user running the application locally on their own desktop computer.

---

## 2. Platform & Architecture

### 2.1 Platform
- **Primary:** Local desktop application, single user, works fully offline.
- **Nice-to-have (P2):** A mobile-friendly experience. Not a priority; it should not drive v1 design decisions, but avoid choices that make it impossible later.

### 2.2 Technology Stack — **[DECIDE]**
Language, framework, UI toolkit, and database are intentionally **not yet chosen**. Whatever is chosen should satisfy:
- Runs locally on desktop with no server or internet connection required
- Embedded/local data storage (single file or local directory) that is easy to back up
- Supports schema migrations as the data model evolves
- Has a reasonable charting option for reports
- Has a good testing story (unit + integration)
- Fits the learning goals (to be defined when choosing the stack)

### 2.3 Core Design Principles (stack-independent)
- **Money is stored as integer minor units** (e.g. cents), never floating point.
- **Every amount is tied to a currency** via its account, even though v1 assumes a single currency. This avoids a painful migration if multi-currency support is added later.
- **Balances are derived** from transactions (or verified against them), never edited directly.
- **Business logic is separated from the UI** so it can be tested independently, and so another front end (e.g. mobile) could be added later.

---

## 3. Data Model

### Account
| Field | Type | Notes |
|---|---|---|
| id | int | PK |
| name | string | "Chase Checking" |
| type | enum | checking, savings, credit_card, cash, loan, other |
| currency | string | ISO 4217; defaults to the app's base currency |
| opening_balance | int (minor units) | |
| archived | bool | hide closed accounts without deleting history |

### Transaction
| Field | Type | Notes |
|---|---|---|
| id | int | PK |
| account_id | FK → Account | |
| date | date | |
| amount | int (minor units) | negative = outflow, positive = inflow |
| payee | string | |
| category_id | FK → Category, nullable | null = uncategorized |
| notes | string, nullable | |
| tags | many-to-many | optional |
| transfer_id | nullable | links the two sides of a transfer between accounts |
| cleared | bool | for reconciliation |
| import_id | FK → Import, nullable | which CSV import created it (null = manual entry) |
| import_fingerprint | string, nullable | hash of date + amount + payee + account for duplicate detection |
| created_at / updated_at | timestamp | |

### Category
| Field | Type | Notes |
|---|---|---|
| id | int | PK |
| name | string | "Groceries" |
| parent_id | nullable FK | one level of nesting (e.g. Food → Groceries) |
| kind | enum | income, expense |

### Import
| Field | Type | Notes |
|---|---|---|
| id | int | PK |
| account_id | FK → Account | |
| file_name | string | |
| imported_at | timestamp | |
| row_count / skipped_count | int | for an import summary; allows undoing an import |

### CSV Import Profile
Saved column mapping per bank format (date column + date format, amount column(s) — single signed column or separate debit/credit columns, payee, notes, header rows to skip, decimal/thousands separators).

### Categorization Rule
Condition (e.g. payee contains / equals / regex, optional amount range) → category. Applied in priority order during import and optionally on manual entry.

### Budget — **[DECIDE: shape depends on budgeting style, see §4.5]**
Placeholder: category + month (YYYY-MM) + amount.

### Recurring Transaction (P1)
Template + schedule (weekly / monthly / yearly) that generates transactions or reminders.

### Savings Goal (P2)
Name, target amount, target date, linked account or manual contributions.

---

## 4. Functional Requirements

Priority: **P0** = MVP, **P1** = soon after, **P2** = nice to have.

### 4.1 Accounts
- P0: Create, edit, archive accounts
- P0: Show current balance (opening balance + sum of transactions)
- P1: Reconcile against a statement balance

### 4.2 Manual Transaction Entry
- P0: Add / edit / delete a transaction
- P0: Keyboard-friendly quick-add form (sensible defaults: today's date, last-used account)
- P0: List transactions with filters (date range, account, category, amount, text search)
- P0: Sort and paginate
- P1: Payee autocomplete, suggesting the category last used for that payee
- P1: Transfers between accounts (not counted as income/expense)
- P1: Split a transaction across multiple categories
- P1: Bulk edit (re-categorize many at once)

### 4.3 CSV Import / Export
- P0: Import a CSV file into a chosen account
- P0: Column-mapping step with a preview of parsed rows before committing
- P0: Save mappings as reusable import profiles per bank
- P0: Duplicate detection (flag rows matching existing transactions; user chooses to skip or keep)
- P0: Clear error reporting for rows that fail to parse (bad dates, amounts)
- P0: Export all transactions (or a filtered set) to CSV
- P1: Undo an entire import
- P1: Auto-categorization rules applied during import
- P2: OFX/QFX file import (file-based only — not bank sync)

### 4.4 Categories
- P0: Default category set seeded on first run; user can add/rename/delete
- P0: Deleting a category requires reassigning its transactions
- P1: Subcategories

### 4.5 Budgets — **[DECIDE: budgeting style]**
Budgeting style is undecided. Candidate approaches:

| Style | How it works | Data-model impact |
|---|---|---|
| **Traditional category budgets** | Set a spending limit per category per month; track spent vs. limit | Simple: category + month + amount |
| **Zero-based / envelope** | Assign all income to categories until "to be budgeted" is zero; unspent money rolls over | Needs rollover balances, "ready to assign" pool, moving money between categories |

Requirements common to either style:
- P0: Show budgeted vs. spent vs. remaining per category for a selected month
- P1: Copy last month's budget
- P1: Over-budget indicators

Because budgets are scheduled after the core ledger and import (see §7), this decision can be deferred until that milestone.

### 4.6 Reports & Dashboard
- P0: Dashboard — total balance, this month's income vs. expenses, top spending categories, recent transactions
- P0: Spending by category (chart) for a selected period
- P1: Income vs. expenses over time (monthly)
- P1: Net worth over time
- P2: Cash-flow forecast based on recurring transactions

### 4.7 Recurring Transactions & Goals
- P1: Recurring transactions with an upcoming-bills list
- P2: Savings goals with progress

### 4.8 Multi-Currency (P2)
- Accounts in different currencies
- Manually entered exchange rates (no live rate fetching required)
- Reports converted to a base currency

### 4.9 Mobile (P2)
- A mobile-friendly way to view data and enter transactions. Approach TBD (e.g. responsive UI, separate app sharing the core logic).

---

## 5. Non-Functional Requirements

- **Correctness:** Integer minor units for all money; balances always derived or verified against transactions.
- **Privacy:** All data stored locally; the app makes no network requests in normal operation; no telemetry/analytics.
- **Security:** Optional encryption of the data file at rest **[DECIDE]**.
- **Backups:** One-click backup of the data file and restore from backup; full CSV export so data is never locked in.
- **Data safety:** Destructive actions (delete account, undo import) require confirmation.
- **Performance:** Transaction list loads in < 500 ms with 50k transactions.
- **Usability:** Keyboard-friendly entry; works well at typical desktop window sizes.
- **Accessibility:** Sufficient color contrast; charts have a table/text fallback.
- **Testing:** Unit tests for balance, budget, and CSV parsing logic; integration tests against the data layer; a folder of sample CSV fixtures from different bank formats.

---

## 6. UI — Main Screens
1. **Dashboard** — balances, month summary, budget health, recent transactions
2. **Transactions** — filterable table + quick-add form
3. **Import** — choose file + account → map columns → preview & resolve duplicates → confirm → summary
4. **Accounts** — list with balances; account detail = filtered transactions
5. **Budgets** — month picker, per-category progress (layout depends on budgeting style)
6. **Reports** — charts with period selector
7. **Settings** — categories, categorization rules, import profiles, base currency, backup/restore, export

---

## 7. Milestones

| Milestone | Scope |
|---|---|
| **M0 – Setup** | Choose stack, project skeleton, data storage + migrations, linting, test setup |
| **M1 – Core ledger** | Accounts, categories, manual transaction entry, filtering, balances |
| **M2 – CSV import/export** | Import with column mapping, preview, saved profiles, duplicate detection, CSV export |
| **M3 – Dashboard & reports** | Dashboard, spending-by-category chart, backup/restore |
| **M4 – Budgets** | Decide budgeting style, then implement budgets |
| **M5 – Enhancements** | Transfers, splits, categorization rules, undo import, reports over time, recurring transactions |
| **M6 – Nice-to-haves** | Savings goals, multi-currency, forecasting, OFX import, mobile |

---

## 8. Decisions Log

| Date | Decision |
|---|---|
| 2026-10-10 | Single user, runs locally on a desktop computer. No hosting or authentication. |
| 2026-10-10 | Mobile support is a nice-to-have (P2), not a priority. |
| 2026-10-10 | Transactions come only from manual entry and CSV import. Bank sync is permanently out of scope. |
| 2026-10-10 | Multi-currency is a nice-to-have (P2); the data model keeps a currency field from the start. |
| 2026-10-10 | Learning is one of the project's goals. Language and framework are deliberately left undecided for now. |

## 9. Open Questions
1. **Technology stack:** language, framework/UI toolkit, data storage (decide in M0).
2. **Learning goals:** what specifically to learn (e.g. a particular language, UI paradigm, database design, testing practices); this should guide the stack choice.
3. **Budgeting style:** traditional vs. zero-based/envelope (decide by M4).
4. **Encryption at rest:** needed for the local data file?
5. **Default categories:** what starting set to seed.
