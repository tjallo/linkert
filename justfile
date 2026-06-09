run:
    cargo run

db-fresh:
    docker compose down -v && docker compose up -d && sleep 2 && sqlx migrate run

watch:
    cargo watch -x run

build:
    cargo build

check:
    cargo check

test:
    cargo test
