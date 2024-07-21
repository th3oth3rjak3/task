//! The `errors` module contains all of the error types
//! for the application.

/// `TaskError` is an error enum that represents
/// all possible failures for the task application.
#[derive(Debug)]
pub enum TaskError {
    /// `AddError` is used when an unexpected error occurs when adding a new
    /// task to the task list.
    AddError(String),
    /// `NumberOutOfRange` is used when an item number is not a valid item number.
    /// For example, if the user enters `task do 12` but there are only 5 items.
    /// This is not used when a number is the wrong sign. For example, do not use
    /// this error when expecting a u8, but the user enters a negative number, those
    /// should be handled during input validation.
    NumberOutOfRange(String),
    /// `FetchError` is an error that is returned when a database error occurs when
    /// fetching items.
    FetchError,
}
