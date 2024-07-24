//! The `models` module contains all of the database models for use in the application.

use chrono::NaiveDateTime;
use diesel::prelude::*;

/// `Task` represents a basic unit of work that a user wants
/// to accomplish.
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    /// `id` is the database id used to uniquely identify a task.
    pub id: i32,
    /// `description` is the actual task to be completed.
    pub description: String,
    /// `complete_date` is the date and time the item was marked done.
    pub complete_date: Option<NaiveDateTime>,
}
