CREATE TABLE posts
(
    id           CHAR(36) PRIMARY KEY,
    content      VARCHAR(1000) NOT NULL,
    author_id    CHAR(36)      NOT NULL,
    created_at   TIMESTAMP          DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP          DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    published_at TIMESTAMP     NULL DEFAULT NULL,
    deleted_at   TIMESTAMP     NULL DEFAULT NULL
);

CREATE INDEX author_id_index ON posts (author_id)