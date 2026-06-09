CREATE TABLE urls (
    id BIGSERIAL PRIMARY KEY,
    stub VARCHAR(32) NOT NULL UNIQUE,
    original_url TEXT NOT NULL,
    user_id BIGINT REFERENCES users(id),
    created_at TIMESTAMP DEFAULT NOW(),
    invalid_by TIMESTAMP
)