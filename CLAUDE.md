# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Whitelist is a booking whitelist application — a JSON API for managing whitelist entries (phone numbers, names, expiration). Built with Rust + Axum, backed by PostgreSQL.

## Tech Stack

- **Rust** with Tokio async runtime
- **Axum 0.7** — web framework (macros feature)
- **SQLx 0.7** — async PostgreSQL driver (runtime-tokio, postgres, uuid features)
- **Serde** — JSON serialization
- **tower-http** — CORS and tracing middleware
- **dotenvy** — loads `.env` for `DATABASE_URL`

## Build & Run Commands

```bash
cargo build              # Build the project
cargo check              # Type-check without building
cargo run                # Run the API server (listens on 127.0.0.1:3030)
cargo test               # Run tests
cargo clippy             # Lint
cargo fmt                # Format code
```

Requires `DATABASE_URL` environment variable (PostgreSQL). Copy `.env.example` to `.env` and configure.

## Git Workflow

- **Atomic commits** with short, unsigned commit messages — no `Co-Authored-By` trailers, no signing
- Local git operations (add, commit, branch, log, diff, etc.) are fine to run freely
- **Never push or fetch** — remote operations require the user's SSH key password; the user handles these manually
- Current feature branch: `init`

## Architecture

```
src/
├── main.rs              # Server setup: router, CORS, tracing, listener on :3030
├── error.rs             # AppError enum (Database, NotImplemented, NotFound) → JSON responses
├── models/
│   └── mod.rs           # WhitelistEntry, CreateWhitelistEntry, ApiResponse<T>
├── db/
│   ├── mod.rs
│   └── pool.rs          # PgPool creation from DATABASE_URL
└── routes/
    ├── mod.rs
    ├── health.rs         # GET /health
    └── whitelist.rs      # CRUD: GET/POST /api/whitelist, GET/DELETE /api/whitelist/:id
```

SQL migrations live in `migrations/` (not managed by sqlx-cli yet, applied manually).

## API Endpoints

| Method | Endpoint | Status |
|--------|----------|--------|
| GET | `/health` | Working |
| GET | `/api/whitelist` | Returns empty array (DB not wired) |
| POST | `/api/whitelist` | Returns 501 |
| GET | `/api/whitelist/:id` | Returns 501 |
| DELETE | `/api/whitelist/:id` | Returns 501 |

All responses use the `ApiResponse<T>` wrapper: `{ success, data, error }`.

## Key Patterns

- **Error handling**: Route handlers return `Result<Json<ApiResponse<T>>, AppError>`. `AppError` implements `IntoResponse` to produce consistent JSON error responses.
- **CORS**: Wide open (`Any` origin/methods/headers) for frontend development.
- **Database pool**: Created via `db::pool::create_pool()`, not yet injected into routes as Axum state.

## Implementation Spec

`strike-init.md` contains the full scaffolding spec with step-by-step implementation instructions and future phase plans (working CRUD, frontend, email verification, calendar sync).
