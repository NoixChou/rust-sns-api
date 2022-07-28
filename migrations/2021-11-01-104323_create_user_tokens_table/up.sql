CREATE TABLE user_tokens
(
    token      CHAR(68) PRIMARY KEY,
    user_id    CHAR(36)  NOT NULL,
    expired_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP          DEFAULT NULL
);

CREATE INDEX ON user_tokens (user_id);
ALTER TABLE user_tokens ADD FOREIGN KEY (user_id) REFERENCES user_credentials(id);
CREATE TRIGGER user_tokens_set_updated_at_trigger
    BEFORE UPDATE
    ON user_tokens
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at();
