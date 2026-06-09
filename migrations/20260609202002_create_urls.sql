CREATE TABLE urls (
    id BIGSERIAL PRIMARY KEY,
    stub VARCHAR(32) NOT NULL UNIQUE,
    original_url TEXT NOT NULL,
    user_id BIGINT NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    invalid_by TIMESTAMPTZ
);

CREATE OR REPLACE FUNCTION update_invalid_by_after_user_delete()
    RETURNS TRIGGER
    LANGUAGE plpgsql
AS $$
BEGIN
    UPDATE urls
    SET invalid_by = NOW()
    WHERE user_id = OLD.id;
    RETURN OLD;
END
$$;

CREATE TRIGGER after_user_delete
    AFTER DELETE
    ON users
    FOR EACH ROW
    EXECUTE PROCEDURE update_invalid_by_after_user_delete();
