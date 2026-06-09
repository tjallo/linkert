# URL Shortener — Project Plan

## Goal
Self-hosted URL shortener with JWT auth, Redis for fast redirect lookups, and PostgreSQL as persistent storage.

---

## Components

### 1. Central Server (this repo)
- HTTP server (axum)
- JWT-based auth (register + login)
- Redis for fast `code → url` lookups + click counters
- PostgreSQL as source of truth (users, URLs, stats)
- sqlx migrations for schema management

---

## Architecture

```
Client
  │
  ├─ POST /auth/register   → create user (Postgres)
  ├─ POST /auth/login      → returns JWT
  │
  ├─ POST /urls            → create short URL (JWT required)
  │                           → store in Postgres
  │                           → cache in Redis
  │
  ├─ GET  /urls            → list user's own URLs (JWT required, Postgres)
  │
  ├─ GET  /s/{code}        → redirect (Redis lookup, fallback to Postgres)
  │                           → increment click counter (Redis)
  │
  └─ GET  /urls/{code}/stats → click count + metadata (JWT required)
```

---

## Storage Design

**Redis** — fast path
- `url:{code}` → original URL string (with optional TTL for expiring links)
- `clicks:{code}` → integer counter (INCR)

**PostgreSQL** — source of truth
- `users` table: id, username, password hash, created_at
- `urls` table: id, user_id, code, original_url, created_at, expires_at
- `clicks` table (optional): code, timestamp (for detailed analytics)

---

## Phases

### Phase 1 — Project Setup ✅
- Cargo dependencies
- Basic axum server
- Module structure

### Phase 2 — Data Modeling ✅
- Structs + serde derives
- Request/response types

### Phase 3 — PostgreSQL + Migrations
- Set up sqlx with Postgres
- Install sqlx-cli
- Write migrations: users + urls tables
- Understand compile-time query checking with `sqlx::query!`

### Phase 4 — Redis Integration
- Connection pool setup
- Store/retrieve short URL codes
- Click counter with INCR
- Cache invalidation strategy

### Phase 5 — Auth (JWT)
- `POST /auth/register` — hash password, store user
- `POST /auth/login` — verify password, return signed JWT
- Axum extractor or middleware to validate JWT on protected routes
- Understand tower middleware vs axum extractors for auth

### Phase 6 — URL Routes
- `POST /urls` — generate short code, store in Postgres + Redis
- `GET /s/{code}` — Redis lookup, fallback to Postgres, redirect
- `GET /urls` — list authenticated user's URLs
- `GET /urls/{code}/stats` — click count + metadata

### Phase 7 — Polish (optional)
- Custom slugs
- Expiring URLs (Redis TTL + Postgres expires_at)
- Detailed click analytics
- Simple HTML frontend

---

## Key Crates
| Crate | Purpose |
|---|---|
| `tokio` | Async runtime |
| `axum` | HTTP server |
| `serde` / `serde_json` | JSON serialization |
| `sqlx` | Async Postgres + migrations |
| `redis` | Redis client |
| `jsonwebtoken` | JWT encode/decode |
| `bcrypt` or `argon2` | Password hashing |
| `nanoid` or `rand` | Short code generation |

---

## Auth Flow
1. User registers with username + password
2. Password hashed (never stored plain)
3. Login returns signed JWT
4. JWT included as `Authorization: Bearer <token>` on protected routes
5. Server validates JWT signature + expiry on each request
