-- Your SQL goes here
CREATE TABLE logins (
    id UUID PRIMARY KEY,
    email VARCHAR UNIQUE NOT NULL,
    hash BYTEA NOT NULL,
    salt BYTEA NOT NULL
)
