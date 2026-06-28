run:
  cargo run

db-fresh:
  docker compose down -v && docker compose up -d --wait && sqlx migrate run

watch:
  cargo watch -x run

build:
  cargo build

check:
  cargo check

test:
  cargo test

feature-test:
  just db-fresh
  deno test --allow-net

openapi:
  curl -s localhost:3000/openapi.json > api.json

gen-types:
  just openapi
  deno task gen-types
