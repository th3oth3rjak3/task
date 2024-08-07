//! `commands` is a module where the implementation for
//! all of the commands is kept. This module is used
//! by main during argument parsing from the command line.

use clap::{Args, Subcommand};

/// `AddArgs` is the set of arguments required for an `Add` command.
#[derive(Args)]
pub struct AddArgs {
    /// A space separated list of words that make up a single task.
    #[arg(required = true)]
    pub words: Vec<String>,
}

/// `RemoveArgs` is the set of arguments required for a `Remove` command.
#[derive(Args)]
pub struct RemoveArgs {
    /// A space separated list of task numbers to be deleted.
    #[arg(required = true)]
    pub task_numbers: Vec<u16>,
}

/// `CompleteArgs` is the set of arguments required for a `Complete` command.
#[derive(Args)]
pub struct CompleteArgs; // TODO: decide if sort option is desired here.

/// `FinishTaskArgs` is the set of arguments required for a `FinishTask` command.
#[derive(Args)]
pub struct FinishTaskArgs {
    /// A space separated list of task numbers to mark as completed.
    #[arg(required = true)]
    pub task_numbers: Vec<u16>,
}

/// `ListArgs` is the set of arguments required for a `List` command.
#[derive(Args)]
pub struct ListArgs; // TODO: decide if sort option is wanted here.

/// `ClearArgs` is the set of arguments required for a `Clear` command.
#[derive(Args)]
pub struct ClearArgs; // TODO: decide if we need these for flags like last week, etc.

/// `Commands` is an enum which represents all of the possible
/// commands in the `task` application.
#[derive(Subcommand)]
pub enum Commands {
    /// Adds a task to the list.
    Add(AddArgs),
    /// Removes one or more tasks from the list.
    #[command(name = "rm")]
    Remove(RemoveArgs),
    /// Marks one or more tasks as completed.
    #[command(name = "do")]
    FinishTask(FinishTaskArgs),
    /// Lists all of the completed tasks.
    #[command(name = "cpl")]
    ListCompleted(CompleteArgs),
    /// Lists the incomplete tasks.
    #[command(name = "ls")]
    ListIncomplete(ListArgs),
    /// Clear all the completed tasks.
    #[command(name = "cls")]
    ClearCompleted(ClearArgs),
}
