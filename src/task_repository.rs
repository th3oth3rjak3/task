//! The `task_repository` module implements the TaskRepository
//! trait which is used to handle all database transactions
//! for Tasks in the task list.

use crate::domain::{Task, TaskRepository};
use crate::sqlite;
use anyhow::Result;

impl TaskRepository for sqlite::Database {
    fn completed_tasks(&mut self) -> Result<Vec<Task>> {
        todo!();
    }

    fn incomplete_tasks(&mut self) -> Result<Vec<Task>> {
        todo!();
    }

    fn add(&mut self, task: Task) -> Result<Task> {
        todo!();
    }
}
