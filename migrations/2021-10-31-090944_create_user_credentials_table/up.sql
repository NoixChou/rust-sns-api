CREATE TABLE user_credentials
(
    id            CHAR(36) PRIMARY KEY,
    password_hash VARCHAR(255) NOT NULL,
    email         VARCHAR(255) NOT NULL,
    created_at    TIMESTAMP         DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMP         DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at    TIMESTAMP    NULL DEFAULT NULL
);

ALTER TABLE user_credentials ADD UNIQUE (email);
ALTER TABLE users ADD FOREIGN KEY user_id_foreign (id) REFERENCES user_credentials (id);