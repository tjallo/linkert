CREATE TABLE urls (
    id BIGSERIAL PRIMARY KEY,
    stub VARCHAR(32) NOT NULL UNIQUE,
    original_url TEXT NOT NULL,
    user_id BIGINT REFERENCES users(id) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    invalid_by TIMESTAMP
);

CREATE FUNCTION update_invalid_by_after_user_delete()
    RETURNS TRIGGER
    LANGUAGE 'plpgsql'
AS $BODY$
BEGIN
    UPDATE urls
    SET invalid_by = NOW()
    WHERE user_id = OLD.id;
    RETURN OLD;
END;
$BODY$;

CREATE TRIGGER after_user_delete
    AFTER DELETE
    ON users
    FOR EACH ROW
    EXECUTE PROCEDURE update_invalid_by_after_user_delete;
