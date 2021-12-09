CREATE TABLE users
(
    id           CHAR(36) PRIMARY KEY,
    id_name      VARCHAR(20)  NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    description  VARCHAR(300) NOT NULL,
    birthday     DATE              DEFAULT NULL,
    website      VARCHAR(100) NOT NULL,
    is_private   BOOL              DEFAULT FALSE NOT NULL,
    created_at   TIMESTAMP         DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP         DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at   TIMESTAMP    NULL DEFAULT NULL
);

ALTER TABLE users ADD UNIQUE (id_name);
CREATE INDEX display_name_index ON users (display_name);
