//! The `sqlite` module contains all of the implementation
//! details for creating and migrating the sqlite database.

use std::env;

use diesel::{Connection, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

/// `MIGRATIONS` is a list of migrations for the application and is used to migrate the
/// database any time the application is started and there are pending migrations.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// `Database` is an abstraction around the sqlite database.
pub struct Database {
    /// `connection` is the database connection to use for all queries.
    pub connection: SqliteConnection,
}

impl Database {
    /// `new` creates a new database instance.
    pub fn new() -> Self {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
        let connection = SqliteConnection::establish(&db_url)
            .unwrap_or_else(|_| panic!("Error connecting to database {}", db_url));
        Database { connection }
    }

    /// `migrate` runs any pending database migrations.
    pub fn migrate(&mut self) {
        let _ = self.connection.run_pending_migrations(MIGRATIONS);
    }
}
