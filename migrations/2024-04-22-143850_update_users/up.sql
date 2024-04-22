-- Your SQL goes here
CREATE TABLE users (
  id TEXT PRIMARY KEY UNIQUE,
  username TEXT NOT NULL UNIQUE,
  hashed_password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

SELECT diesel_manage_updated_at('users');