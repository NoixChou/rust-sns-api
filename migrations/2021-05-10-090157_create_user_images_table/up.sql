CREATE TABLE user_images
(
    id         CHAR(36) PRIMARY KEY,
    user_id    CHAR(36) REFERENCES users (id),
    object_key VARCHAR(200) NOT NULL,
    image_type VARCHAR(255) NOT NULL,
    created_at TIMESTAMP    NULL DEFAULT NULL,
    updated_at TIMESTAMP    NULL DEFAULT NULL,
    deleted_at TIMESTAMP    NULL DEFAULT NULL
);

CREATE INDEX user_id_index ON user_images (user_id);