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
use domain::Task;
use dotenvy;
use sqlite::Database;

pub mod commands;
pub mod domain;
pub mod handlers;
pub mod models;
/// thing.
pub mod schema;
pub mod sqlite;
pub mod task_repository;

/// Task is a simple command line tool to manage personal tasks.
#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let app = Cli::parse();
    let mut db = Database::new();
    db.migrate();
    match app.commands {
        Commands::Add(words) => {
            let outcome = handlers::add(&mut db, words.words);
            todo!();
        }
        Commands::Remove(tasks) => {
            let outcomes = handlers::remove(tasks.task_numbers);
            todo!();
        }
        Commands::ListCompleted(_) => {
            let tasks = handlers::get_completed_tasks(&mut db)?;
            show_completed_tasks(tasks);
        }
        Commands::ListIncomplete(_) => {
            let tasks = handlers::get_incomplete_tasks(&mut db)?;
            show_incomplete_tasks(tasks);
        }
        Commands::Do(tasks) => {
            let outcomes = handlers::mark_complete(tasks.task_numbers);
            todo!();
        }
    }

    Ok(())
}

fn show_completed_tasks(tasks: Vec<Task>) {
    println!("Number of completed tasks: {}", tasks.len());
    todo!();
}

fn show_incomplete_tasks(tasks: Vec<Task>) {
    println!("Number of incomplete tasks: {}", tasks.len());
    todo!();
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
