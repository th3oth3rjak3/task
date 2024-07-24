//! `handlers` is a module where all of the individual handlers are kept
//! for each command. Handlers are functions that are executed depending
//! on which command was requested by the user.

use crate::{
    domain::{Task, TaskRepository},
    errors::TaskError,
};
use std::collections::HashSet;

/// `add` adds a single task to the task list.
///
/// A list of strings is allowed so that the user doesn't have
/// to surround words with quotes on the command line. All strings are
/// assumed to belong to a single sentence task.
///
/// ### Args
/// * `words` - The list of words that a user typed which form
/// the contents of the task. e.g. `["walk", "the", "dog"]` or `["walk the dog"]`
///
/// ### Returns
/// * `Task` - The task that was added successfully.
/// * `TaskError` - When an error occurs, this is returned to tell the user what went wrong.
pub fn add(task_repo: &mut impl TaskRepository, words: Vec<String>) -> Result<Task, TaskError> {
    // trim/join words into sentence
    let description = words
        .into_iter()
        .map(|word| word.trim().to_string())
        .collect::<Vec<String>>()
        .join(" ");

    // generate new task with Task::new()
    let task = Task::new(description);

    // save into database using repo.
    task_repo.add(task)
}

/// `remove` deletes 1 or more items from the task list.
///
/// ### Args
/// * `numbers` - A list of task numbers provided by the user to delete
/// tasks from the application. These are the numbers that the user sees in the
/// terminal after running the `list` command.
///
/// ### Errors
/// * When a provided item number is a valid u16, but not a valid
/// task number, it will return a `TaskError::NumberOutOfRange`
///
/// ### Returns
/// * `Task` - The task that was removed successfully.
/// * `TaskError` - When an error occurs, this is returned to tell the user what went wrong.
pub fn remove(numbers: Vec<u16>) -> Vec<Result<Task, TaskError>> {
    // TODO: filter the list of items to ensure that the list is unique.
    // This prevents trying to remove an item twice.
    numbers.iter().for_each(|number| println!("{:?}", number));
    todo!();
}

/// `mark_complete` marks all items in the provided list as finished.
///
/// ### Args
/// `numbers` - The list of task numbers provided by the user to mark as finished.
/// These are the numbers that the user sees in the terminal after running the `list`
/// command.
///
/// ### Errors
/// * When a provided item number is a valid u16, but not a valid
/// task number, it will return a `TaskError::NumberOutOfRange`
///
/// ### Returns
/// * `Task` - The task that was successfully marked as done.
/// * `TaskError` - When an error occurs, this is returned to tell the user what went wrong.
pub fn mark_complete(numbers: Vec<u16>) -> Vec<Result<Task, TaskError>> {
    // TODO: filter the list of items to ensure that the list is unique.
    // This prevents trying to mark an item completed twice.
    numbers.iter().for_each(|number| println!("{:?}", number));
    todo!();
}

/// `get_completed_tasks` gets a list of all tasks which have no
/// completion date.
pub fn get_completed_tasks(task_repo: &mut impl TaskRepository) -> Result<Vec<Task>, TaskError> {
    task_repo.completed_tasks()
}

/// `get_incomplete_tasks` gets a list of all tasks which have a completion date.
pub fn get_incomplete_tasks(task_repo: &mut impl TaskRepository) -> Result<Vec<Task>, TaskError> {
    task_repo.incomplete_tasks()
}

/// `remove_duplicates` filters out any duplicate numbers to prevent
/// processing an item number more than once.
///
/// ### Args
/// * `numbers` - a list of task numbers to make unique.
///
/// ### Returns
/// * `HashSet<u16>` - a set of task numbers which is unique.
fn remove_duplicates(numbers: Vec<u16>) -> HashSet<u16> {
    numbers.into_iter().collect()
}
