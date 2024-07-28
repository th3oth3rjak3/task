//! The `domain` module is used to document all of the application
//! domain specific types.

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

/// `Task` represents a basic unit of work that a user wants
/// to accomplish.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    /// `id` is the database id used to uniquely identify a task.
    pub id: i64,
    /// `description` is the actual task to be completed.
    pub description: String,
    /// `complete_date` is the date and time the item was marked done.
    pub complete_date: Option<DateTime<Utc>>,
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

    /// `from_record` creates a new task from an entire database row.
    pub fn from_record(id: i64, description: String, date: Option<NaiveDateTime>) -> Self {
        let complete_date = date.map(|dt| dt.and_utc());

        Task {
            id,
            description,
            complete_date,
        }
    }
}

/// `TaskRepository` is a trait which contains all of the database
/// operations for managing tasks in the task list.
#[async_trait]
pub trait TaskRepository {
    /// `completed_tasks` returns a list of completed tasks.
    async fn completed_tasks(&mut self) -> Result<Vec<Task>>;

    /// `incomplete_tasks` returns a list of incomplete tasks.
    async fn incomplete_tasks(&mut self) -> Result<Vec<Task>>;

    /// `add` inserts a new task into the database.
    async fn add(&mut self, task: Task) -> Result<Task>;

    /// `mark_complete` sets the date and time the task was finished.
    async fn mark_complete(&mut self, task_id: i64) -> Result<Task>;

    /// `delete_task` removes the task from the database.
    async fn delete_task(&mut self, task_id: i64) -> Result<()>;
}
