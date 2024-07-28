-- Add up migration script here
CREATE TABLE IF NOT EXISTS Tasks (
    id INTEGER PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    complete_date TIMESTAMP NULL
);