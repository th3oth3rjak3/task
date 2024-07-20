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
#![deny(unsafe_code)]
#![warn(dead_code)]
#![warn(unused_variables)]

use clap::Parser;
use commands::Commands;
use errors::TaskError;

pub mod commands;
pub mod errors;
pub mod handlers;

/// Task is a simple command line tool to manage personal tasks.
#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let args = Cli::parse();
    let result = match args.command {
        Commands::Add(words) => handlers::add(words.words),
    };

    match result {
        Ok(msg) => {
            println!("{}", msg);
        }
        Err(TaskError::AddError(msg)) => {
            println!("{}", msg);
        }
        Err(TaskError::NumberOutOfRange(msg)) => {
            println!("{}", msg);
        }
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
