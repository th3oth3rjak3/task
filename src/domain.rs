//! The `domain` module is used to document all of the application
//! domain specific types.

use chrono::{DateTime, Utc};


/// `Task` represents a basic unit of work that a user wants
/// to accomplish.
#[derive(Debug, Clone, )]
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
}
