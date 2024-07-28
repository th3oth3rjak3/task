//! The `task_repository` module implements the TaskRepository
//! trait which is used to handle all database transactions
//! for Tasks in the task list.

use crate::{domain::Task, sqlite};
use anyhow::{Error, Result};
use async_trait::async_trait;
use chrono::Utc;
use sqlx::Connection;

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

#[async_trait]
impl TaskRepository for sqlite::Database {
    async fn completed_tasks(&mut self) -> Result<Vec<Task>> {
        sqlx::query!(
            "
            SELECT *
            FROM Tasks 
            WHERE complete_date IS NOT NULL;
            ",
        )
        .map(|row| Task::from_record(row.id, row.description, row.complete_date))
        .fetch_all(&mut self.connection)
        .await
        .map_err(Error::new)
    }

    async fn incomplete_tasks(&mut self) -> Result<Vec<Task>> {
        sqlx::query!(
            "
            SELECT id, description, complete_date
            FROM Tasks
            WHERE complete_date IS NULL;
            "
        )
        .map(|row| Task::from_record(row.id, row.description, row.complete_date))
        .fetch_all(&mut self.connection)
        .await
        .map_err(Error::new)
    }

    async fn add(&mut self, task: Task) -> Result<Task> {
        let complete_date = &task.complete_date.map(|cpl| cpl.to_string());

        sqlx::query!(
            "
            INSERT INTO Tasks (description, complete_date) 
            VALUES (?, ?)
            ",
            task.description,
            complete_date,
        )
        .execute(&mut self.connection)
        .await
        .map(|res| Task {
            id: res.last_insert_rowid(),
            ..task
        })
        .map_err(Error::new)
    }

    async fn mark_complete(&mut self, task_id: i64) -> Result<Task> {
        let mut tx = self.connection.begin().await.map_err(Error::new)?;

        let current_date_time = Utc::now().naive_utc().to_string();

        sqlx::query!(
            "
                UPDATE Tasks
                SET complete_date = ?
                WHERE id = ?
            ",
            current_date_time,
            task_id
        )
        .execute(&mut *tx)
        .await
        .map_err(Error::new)?;

        tx.commit().await.map_err(Error::new)?;

        sqlx::query!(
            "SELECT id, description, complete_date FROM Tasks WHERE id = ?",
            task_id
        )
        .map(|res| Task::from_record(res.id, res.description, res.complete_date))
        .fetch_one(&mut self.connection)
        .await
        .map_err(Error::new)
    }

    async fn delete_task(&mut self, task_id: i64) -> Result<()> {
        sqlx::query!(
            "
                DELETE FROM Tasks
                WHERE id = ?;
            ",
            task_id
        )
        .execute(&mut self.connection)
        .await
        .map(|_| ())
        .map_err(Error::new)
    }
}
