//! `commands` is a module where the implementation for
//! all of the commands is kept. This module is used
//! by main during argument parsing from the command line.

use clap::{Args, Subcommand};

/// `AddArgs` is the set of arguments required for an `Add` command.
#[derive(Args)]
pub struct AddArgs {
    /// `words` is a list of words that make up the sentence of a single task.
    /// It is marked required to prevent empty tasks.
    #[arg(required = true)]
    pub words: Vec<String>,
}

/// `Commands` is an enum which represents all of the possible
/// commands in the `task` application.
#[derive(Subcommand)]
pub enum Commands {
    /// Adds a task to the list.
    Add(AddArgs),
}
