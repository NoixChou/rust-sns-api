CREATE TABLE posts
(
    id CHAR(36) PRIMARY KEY,
    message VARCHAR(1000) NOT NULL,
    author_id CHAR(36) NOT NULL,
    created_at TIMESTAMP NULL DEFAULT NULL,
    updated_at TIMESTAMP NULL DEFAULT NULL,
    published_at TIMESTAMP NULL DEFAULT NULL,
    deleted_at TIMESTAMP NULL DEFAULT NULL
);

CREATE INDEX author_id_index ON posts(author_id)