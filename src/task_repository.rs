//! The `task_repository` module implements the TaskRepository
//! trait which is used to handle all database transactions
//! for Tasks in the task list.

use crate::sqlite;
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use chrono::Utc;
use entity::task::{self};

use sea_orm::{entity::*, query::*};

/// `TaskRepository` is a trait which contains all of the database
/// operations for managing tasks in the task list.
#[async_trait]
pub trait TaskRepository {
    /// `completed_tasks` returns a list of completed tasks.
    async fn completed_tasks(&self) -> Result<Vec<task::Model>>;

    /// `incomplete_tasks` returns a list of incomplete tasks.
    async fn incomplete_tasks(&self) -> Result<Vec<task::Model>>;

    /// `add` inserts a new task into the database.
    async fn add(&self, description: String) -> Result<task::Model>;

    /// `mark_complete` sets the date and time the task was finished.
    async fn mark_complete(&self, task_id: i32) -> Result<task::Model>;

    /// `delete_task` removes the task from the database.
    async fn delete_task(&self, task_id: i32) -> Result<()>;

    /// `clear_completed` deletes the completed items from the database.
    async fn clear_completed(&self) -> Result<()>;
}

#[async_trait]
impl TaskRepository for sqlite::Database {
    async fn completed_tasks(&self) -> Result<Vec<task::Model>> {
        task::Entity::find()
            .filter(task::Column::CompleteDate.is_not_null())
            .all(&self.connection)
            .await
            .map_err(Error::new)
    }

    async fn incomplete_tasks(&self) -> Result<Vec<task::Model>> {
        task::Entity::find()
            .filter(task::Column::CompleteDate.is_null())
            .all(&self.connection)
            .await
            .map_err(Error::new)
    }

    async fn add(&self, description: String) -> Result<task::Model> {
        let new_task = task::ActiveModel {
            id: NotSet,
            description: Set(description),
            complete_date: Set(None),
        };

        new_task.insert(&self.connection).await.map_err(Error::new)
    }

    async fn mark_complete(&self, task_id: i32) -> Result<task::Model> {
        let found = task::Entity::find_by_id(task_id)
            .one(&self.connection)
            .await
            .map_err(Error::new)?;
        if let Some(task) = found {
            let mut active_task: task::ActiveModel = task.into();
            active_task.complete_date = Set(Some(Utc::now()));
            return active_task
                .update(&self.connection)
                .await
                .map_err(Error::new);
        }

        Err(anyhow!("Task with id '{}' not found.", task_id))
    }

    async fn delete_task(&self, task_id: i32) -> Result<()> {
        task::Entity::delete_by_id(task_id)
            .exec(&self.connection)
            .await
            .map(|_| ())
            .map_err(Error::new)
    }

    async fn clear_completed(&self) -> Result<()> {
        task::Entity::delete_many()
            .filter(task::Column::CompleteDate.is_not_null())
            .exec(&self.connection)
            .await
            .map(|_| ())
            .map_err(Error::new)
    }
}
