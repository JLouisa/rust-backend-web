CREATE TABLE users (
    user_id                 TEXT PRIMARY KEY NOT NULL,
    username                TEXT UNIQUE NOT NULL,
    hashed_password         TEXT NOT NULL,
    created_on              DATETIME DEFAULT (datetime('now','localtime')),
    updated_on              DATETIME DEFAULT (datetime('now','localtime')),
    active                  BOOLEAN NOT NULL DEFAULT 1
);
CREATE UNIQUE INDEX users_id_idx on book (id)