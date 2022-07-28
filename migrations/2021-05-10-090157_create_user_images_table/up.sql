CREATE TABLE user_images
(
    id         CHAR(36) PRIMARY KEY,
    user_id    CHAR(36) REFERENCES users (id),
    object_key VARCHAR(200) NOT NULL,
    image_type VARCHAR(255) NOT NULL,
    created_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP             DEFAULT NULL
);

CREATE INDEX ON user_images (user_id);
CREATE TRIGGER user_images_set_updated_at_trigger
    BEFORE UPDATE
    ON user_images
    FOR EACH ROW
EXECUTE PROCEDURE set_updated_at();
