//! The `domain` module is used to document all of the application
//! domain specific types.

use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// `Task` represents a basic unit of work that a user wants
/// to accomplish.
#[derive(Serialize, Deserialize)]
pub struct Task {
    /// `id` is the database id used to uniquely identify a task.
    pub id: i64,
    /// `description` is the actual task to be completed.
    pub description: String,
    /// `complete_date` is the date and time the item was marked done.
    pub complete_date: Option<DateTime<Local>>,
}

impl Task {
    /// `new` creates a new task from a description.
    pub fn new(description: String) -> Self {
        Self {
            id: -1,
            description,
            complete_date: None,
        }
    }
}

/// `TaskRepository` is a trait which contains all of the database
/// operations for managing tasks in the task list.
pub trait TaskRepository {
    /// `completed_tasks` returns a list of completed tasks.
    fn completed_tasks(&mut self) -> Result<Vec<Task>>;

    /// `incomplete_tasks` returns a list of incomplete tasks.
    fn incomplete_tasks(&mut self) -> Result<Vec<Task>>;

    /// `add` inserts a new task into the database.
    fn add(&mut self, task: Task) -> Result<Task>;
}
