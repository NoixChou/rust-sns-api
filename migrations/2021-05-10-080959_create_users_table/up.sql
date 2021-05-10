CREATE TABLE users
(
    id CHAR(36) PRIMARY KEY,
    id_name VARCHAR(20) NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    description VARCHAR(300),
    birthday DATE DEFAULT NULL,
    website VARCHAR(100) DEFAULT NULL,
    is_private BOOL DEFAULT FALSE,
    created_at TIMESTAMP NULL DEFAULT NULL,
    updated_at TIMESTAMP NULL DEFAULT NULL,
    deleted_at TIMESTAMP NULL DEFAULT NULL
);

CREATE INDEX id_name_index ON users(id_name);