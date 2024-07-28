//! `task` is a command line application that is used
//! to manage a small list of personal tasks.
//!
//! It is able to handle the following actions:
//! * Add new tasks
//! * Mark tasks as completed
//! * Delete unwanted tasks
//! * Show a list of all incomplete tasks
//! * Show a list of all tasks completed this week organized by day
//! * Show a list of all tasks completed for all time
//! * Clear completed tasks older than this week
//! * Clear completed tasks for all time.
//! * Reset the entire app, removing all entries.

#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![warn(dead_code)]
#![warn(unused_variables)]

use anyhow::Result;
use clap::Parser;
use commands::Commands;
use sqlite::Database;

pub mod commands;
pub mod domain;
pub mod handlers;
pub mod sqlite;
pub mod task_repository;

/// Task is a simple command line tool to manage personal tasks.
#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[async_std::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let app = Cli::parse();

    let mut db = Database::new().await;

    match app.commands {
        Commands::Add(words) => handlers::add(&mut db, words.words)
            .await
            .unwrap_or_else(show_error),
        Commands::Remove(tasks) => handlers::remove(&mut db, tasks.task_numbers)
            .await
            .unwrap_or_else(show_error),
        Commands::ListCompleted(_) => handlers::get_completed_tasks(&mut db)
            .await
            .unwrap_or_else(show_error),
        Commands::ListIncomplete(_) => handlers::get_incomplete_tasks(&mut db)
            .await
            .unwrap_or_else(show_error),
        Commands::FinishTask(tasks) => handlers::mark_complete(&mut db, tasks.task_numbers)
            .await
            .unwrap_or_else(show_error),
        Commands::ClearCompleted(_) => handlers::clear_completed(&mut db)
            .await
            .unwrap_or_else(show_error),
    }

    Ok(())
}

fn show_error(err: anyhow::Error) {
    eprintln!("Unexpected error occurred: {}", err);
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
