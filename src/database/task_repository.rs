//! The `task_repository` module implements the TaskRepository
//! trait which is used to handle all database transactions
//! for Tasks in the task list.

use crate::{domain::Task, errors::TaskError};

use super::sqlite::Database;

/// `TaskRepository` is a trait which contains all of the database
/// operations for managing tasks in the task list.
pub trait TaskRepository {
    /// `completed_tasks` returns a list of completed tasks.
    fn completed_tasks(&mut self) -> Result<Vec<Task>, TaskError>;

    /// `incomplete_tasks` returns a list of incomplete tasks.
    fn incomplete_tasks(&mut self) -> Result<Vec<Task>, TaskError>;

    /// `add` inserts a new task into the database.
    fn add(&mut self, task: Task) -> Result<Task, TaskError>;
}

impl TaskRepository for Database {
    fn completed_tasks(&mut self) -> Result<Vec<Task>, TaskError> {
        todo!();
    }

    fn incomplete_tasks(&mut self) -> Result<Vec<Task>, TaskError> {
        todo!();
    }

    fn add(&mut self, task: Task) -> Result<Task, TaskError> {
        todo!();
    }
}
