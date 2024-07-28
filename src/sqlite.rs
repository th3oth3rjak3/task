//! The `sqlite` module contains all of the implementation
//! details for creating and migrating the sqlite database.

use sea_orm::{prelude::*, ConnectOptions};
use std::time::Duration;

/// `Database` is an abstraction around the sqlite database.
pub struct Database {
    /// `connection` is the database connection to use for all queries.
    pub connection: DatabaseConnection,
}

impl Database {
    /// `new` creates a new database instance.
    pub async fn new() -> Self {
        let connection = get_connection().await;

        use migration::{Migrator, MigratorTrait};

        Migrator::up(&connection, None)
            .await
            .expect("Migrations should succeed");

        Database { connection }
    }
}

async fn get_connection() -> DatabaseConnection {
    // used when there is not local environment variable set.
    let data_url = |_| -> String {
        let dir = dirs::data_dir()
            .expect("Data directory should exist")
            .to_string_lossy()
            .to_string();

        format!("sqlite://{}/tasks.sqlite3?mode=rwc", dir)
    };

    let url = std::env::var("DATABASE_URL").unwrap_or_else(data_url);

    let mut opts = ConnectOptions::new(&url);

    opts.connect_timeout(Duration::from_secs(2))
        .sqlx_logging(false);

    sea_orm::Database::connect(opts)
        .await
        .expect("Database connection to succeed.")
}
