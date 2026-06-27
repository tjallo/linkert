run:
  cargo run

db-fresh:
  docker compose down -v && docker compose up -d --wait && sleep 1 && sqlx migrate run

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
  just run &
  sleep 2
  deno test --allow-net
