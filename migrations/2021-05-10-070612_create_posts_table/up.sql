CREATE TABLE posts
(
    id           CHAR(36) PRIMARY KEY,
    content      VARCHAR(1000) NOT NULL,
    author_id    CHAR(36)      NOT NULL,
    created_at   TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    published_at TIMESTAMP              DEFAULT NULL,
    deleted_at   TIMESTAMP              DEFAULT NULL
);

CREATE INDEX ON posts (author_id);
CREATE TRIGGER posts_set_updated_at_trigger
    BEFORE UPDATE
    ON posts
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at();
