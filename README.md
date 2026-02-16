# Whitelist

A booking whitelist API built with Rust, Axum, and PostgreSQL. Callers submit booking requests; the owner approves or denies them. Approval auto-creates a whitelist entry.

## Requirements

- Rust (stable)
- Docker & Docker Compose (for PostgreSQL)
- `jq` (for test script)

## Setup

```bash
# Start PostgreSQL
docker compose up -d

# Configure environment
cp .env.example .env

# Run the server (migrations run automatically)
cargo run
```

The API listens on `http://localhost:4040`.

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| GET | `/api/whitelist` | List whitelist entries |
| POST | `/api/whitelist` | Create whitelist entry |
| GET | `/api/whitelist/:id` | Get whitelist entry |
| DELETE | `/api/whitelist/:id` | Delete whitelist entry |
| GET | `/api/bookings` | List bookings (optional `?status=` filter) |
| POST | `/api/bookings` | Submit booking request |
| GET | `/api/bookings/:id` | Get booking |
| PATCH | `/api/bookings/:id` | Approve or deny booking |

## Usage Examples

Create a booking:
```bash
curl -X POST localhost:4040/api/bookings \
  -H 'Content-Type: application/json' \
  -d '{"caller_name":"Jane Doe","caller_phone":"555-234-5678","caller_email":"jane@example.com","call_reason":"Schedule repair"}'
```

Approve a booking (auto-creates whitelist entry):
```bash
curl -X PATCH localhost:4040/api/bookings/<id> \
  -H 'Content-Type: application/json' \
  -d '{"status":"approved"}'
```

Deny a booking:
```bash
curl -X PATCH localhost:4040/api/bookings/<id> \
  -H 'Content-Type: application/json' \
  -d '{"status":"denied"}'
```

List pending bookings:
```bash
curl 'localhost:4040/api/bookings?status=pending'
```

## Testing

With the server running:

```bash
./test.sh
```

Runs 12 tests (29 assertions) covering the full booking lifecycle, validation, and error handling.

## Development

```bash
cargo check    # Type-check
cargo clippy   # Lint
cargo fmt      # Format
cargo test     # Unit tests
```
