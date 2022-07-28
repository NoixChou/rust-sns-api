CREATE TABLE users
(
    id           CHAR(36) PRIMARY KEY,
    id_name      VARCHAR(20)  NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    description  VARCHAR(300) NOT NULL,
    birthday     DATE                  DEFAULT NULL,
    website      VARCHAR(100) NOT NULL,
    is_private   BOOL         NOT NULL DEFAULT FALSE,
    created_at   TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at   TIMESTAMP             DEFAULT NULL
);

ALTER TABLE users ADD UNIQUE (id_name);
CREATE INDEX ON users (display_name);
CREATE TRIGGER users_set_updated_at_trigger
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at();
