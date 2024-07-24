//! The `sqlite` module contains all of the implementation
//! details for creating and migrating the sqlite database.

use rusqlite::Connection;

const DB_URL: &str = "tasks.db";

/// `Database` is an abstraction around the sqlite database.
pub struct Database {
    /// `connection` is the database connection to use for all queries.
    pub connection: Connection,
}

impl Database {
    /// `new` creates a new database instance.
    pub fn new() -> Self {
        let connection = Connection::open(DB_URL).expect("Could not open database connection");
        Database { connection }
    }

    /// `migrate` runs any pending database migrations.
    pub fn migrate(&mut self) {
        let create_tasks_table = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/sql/migrations/create_tasks_table.sql"
        ));

        self.connection
            .execute(&create_tasks_table, ())
            .expect("Failed to create required Task table.");
    }
}
