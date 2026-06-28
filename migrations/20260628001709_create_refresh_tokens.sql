-- Add migration script here

CREATE TABLE refresh_tokens (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL REFERENCES users(id),
    refresh_token TEXT NOT NULL,
    user_agent TEXT DEFAULT NULL,
    device_name VARCHAR(32) DEFAULT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '30 days',
    revoked_at TIMESTAMPTZ DEFAULT NULL
);