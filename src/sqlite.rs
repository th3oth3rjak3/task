//! The `sqlite` module contains all of the implementation
//! details for creating and migrating the sqlite database.

use sqlx::{migrate::Migrator, sqlite::SqliteConnectOptions, ConnectOptions, SqliteConnection};

/// `MIGRATOR` is a list of migrations for the application and is used to migrate the
/// database any time the application is started and there are pending migrations.
pub static MIGRATOR: Migrator = sqlx::migrate!();

/// `Database` is an abstraction around the sqlite database.
pub struct Database {
    /// `connection` is the database connection to use for all queries.
    pub connection: SqliteConnection,
}

impl Database {
    /// `new` creates a new database instance.
    pub async fn new() -> Self {
        let mut connection = get_connection().await;

        MIGRATOR
            .run(&mut connection)
            .await
            .expect("Migrations should have ran successfully.");

        Database { connection }
    }
}

#[cfg(debug_assertions)]
async fn get_connection() -> SqliteConnection {
    use std::{env, str::FromStr};

    let url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    SqliteConnectOptions::from_str(&url)
        .unwrap()
        .create_if_missing(true)
        .connect()
        .await
        .expect("Database connection to succeed")
}

#[cfg(not(debug_assertions))]
async fn get_connection() -> SqliteConnection {
    let db_path = dirs::data_dir()
        .expect("cannot find home directory")
        .join("tasks.sqlite3");

    SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true)
        .connect()
        .await
        .expect("Database connection to succeed")
}
