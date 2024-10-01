-- Your SQL goes here

DROP TABLE IF EXISTS numbers;

CREATE TABLE numbers (
    id INTEGER PRIMARY KEY AUTOINCREMENT ,
    digits TEXT NOT NULL,
    carrier TEXT NULL,
    email TEXT NULL,
    international TEXT NULL,
    national TEXT NULL,
    rfc3966 TEXT NULL,
    e164 TEXT NULL,
    is_valid BOOLEAN NOT NULL DEFAULT 0
)