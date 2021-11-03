CREATE TABLE user_tokens
(
    token CHAR(68) PRIMARY KEY,
    user_id CHAR(36) NOT NULL,
    expired_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP NULL DEFAULT NULL
);

CREATE INDEX user_id_index ON user_tokens(user_id);
ALTER TABLE user_tokens ADD FOREIGN KEY user_id_foreign(user_id) REFERENCES user_credentials(id);