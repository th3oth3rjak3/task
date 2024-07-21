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

#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(dead_code)]
#![warn(unused_variables)]

use clap::Parser;
use commands::Commands;
use database::sqlite::Database;
use domain::Task;
use errors::TaskError;

pub mod commands;
pub mod database;
pub mod domain;
pub mod errors;
pub mod handlers;

/// Task is a simple command line tool to manage personal tasks.
#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

fn main() {
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
            let result = handlers::get_completed_tasks(&mut db);

            match result {
                Ok(tasks) => show_completed_tasks(tasks),
                Err(err) => handle_errors(err),
            }
        }
        Commands::ListIncomplete(_) => {
            let result = handlers::get_incomplete_tasks(&mut db);

            match result {
                Ok(tasks) => show_incomplete_tasks(tasks),
                Err(err) => handle_errors(err),
            }
        }
        Commands::Do(tasks) => {
            let outcomes = handlers::mark_complete(tasks.task_numbers);
            todo!();
        }
    }
}

fn show_completed_tasks(tasks: Vec<Task>) {
    println!("Number of completed tasks: {}", tasks.len());
    todo!();
}

fn show_incomplete_tasks(tasks: Vec<Task>) {
    println!("Number of incomplete tasks: {}", tasks.len());
    todo!();
}

fn handle_errors(err: TaskError) {
    todo!();
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
