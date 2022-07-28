CREATE TABLE user_credentials
(
    id            CHAR(36) PRIMARY KEY,
    password_hash VARCHAR(255) NOT NULL,
    email         VARCHAR(255) NOT NULL,
    created_at    TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at    TIMESTAMP             DEFAULT NULL
);

ALTER TABLE user_credentials ADD UNIQUE (email);
ALTER TABLE users ADD FOREIGN KEY (id) REFERENCES user_credentials (id);
CREATE TRIGGER user_credentials_set_updated_at_trigger
    BEFORE UPDATE
    ON user_credentials
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at();
