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
  deno test
