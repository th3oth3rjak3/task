-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    description TEXT NOT NULL,
    complete_date DATETIME
);