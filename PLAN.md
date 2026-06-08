# Homelab Stats Collector — Project Plan

## Goal
Self-hosted metrics system. Agents on each Linux box push stats to a central Rust server. Server stores, authenticates, and exposes metrics.

---

## Components

### 1. Central Server (this repo)
- HTTP server accepting metrics from agents
- JWT-based authentication
- SQLite storage
- JSON API for querying metrics

### 2. Agent (separate crate, later)
- Collects system metrics (CPU, memory, disk, network)
- Authenticates with server (JWT)
- Pushes metrics on interval

---

## Phases

### Phase 1 — Project Structure
- Understand Cargo workspaces vs single crate
- Set up dependencies in Cargo.toml
- Choose async runtime (tokio)
- Choose HTTP framework (axum)

### Phase 2 — HTTP Server Basics
- Get a basic axum server running
- Define routes
- Understand handlers, extractors, state

### Phase 3 — Data Modeling
- Define metric structs
- Derive serde traits for JSON (de)serialization
- Understand ownership when passing data between layers

### Phase 4 — Storage
- Set up SQLx with SQLite
- Write migrations
- Implement insert and query functions
- Understand async database patterns in Rust

### Phase 5 — Authentication (JWT)
- Understand JWT structure (header, payload, signature)
- Issue tokens (login/register endpoint or static secret)
- Validate tokens on protected routes using axum middleware/extractors
- Understand tower middleware vs axum extractors for auth

### Phase 6 — Agent
- New crate (or workspace member)
- Use `sysinfo` crate to read system metrics
- Use `reqwest` to POST metrics
- Schedule with `tokio::time::interval`
- Handle auth token storage/refresh

### Phase 7 — Polish (optional)
- Grafana JSON datasource compatibility
- Simple HTML dashboard
- Multi-host queries
- Token expiry and refresh flow

---

## Key Crates to Learn
| Crate | Purpose |
|---|---|
| `tokio` | Async runtime |
| `axum` | HTTP server |
| `serde` / `serde_json` | JSON serialization |
| `sqlx` | Async database |
| `jsonwebtoken` | JWT encode/decode |
| `sysinfo` | System metrics (agent) |
| `reqwest` | HTTP client (agent) |

---

## Wire Format (JSON)
```
POST /metrics
Authorization: Bearer <token>

{
  "hostname": "speedmeister",
  "timestamp": <unix seconds>,
  "cpu_percent": <float>,
  "mem_used_bytes": <u64>,
  "mem_total_bytes": <u64>,
  "disk_used_bytes": <u64>,
  "disk_total_bytes": <u64>,
  "net_rx_bytes": <u64>,
  "net_tx_bytes": <u64>
}
```

---

## Auth Flow
1. Agent has a pre-shared secret (or username/password)
2. Agent POSTs credentials to `/auth/token`
3. Server returns signed JWT
4. Agent includes JWT in `Authorization: Bearer` header on all metric pushes
5. Server middleware validates JWT before processing metrics
