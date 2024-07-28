//! `handlers` is a module where all of the individual handlers are kept
//! for each command. Handlers are functions that are executed depending
//! on which command was requested by the user.

use anyhow::Result;
use chrono::Local;

use crate::{domain::Task, task_repository::TaskRepository};
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
pub async fn add(task_repo: &mut impl TaskRepository, words: Vec<String>) -> Result<()> {
    // trim/join words into sentence
    let description = words
        .into_iter()
        .map(|word| word.trim().to_string())
        .collect::<Vec<String>>()
        .join(" ");

    // generate new task with Task::new()
    let task = Task::new(description);

    // save into database using repo.
    let new_task = task_repo.add(task).await?;

    println!("\nAdded \"{}\" to your task list. 🖊️", new_task.description);

    Ok(())
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
pub async fn remove(task_repo: &mut impl TaskRepository, numbers: Vec<u16>) -> Result<()> {
    let unique = remove_duplicates(numbers);

    let incomplete = task_repo.incomplete_tasks().await?;

    println!();

    if incomplete.is_empty() {
        println!("There are no tasks to delete. 🤷");
        return Ok(());
    }

    for item_number in unique.iter() {
        if usize::from(*item_number) > incomplete.len() || item_number < &1 {
            println!("Invalid task number: {} 🤕", item_number);
            continue;
        }

        let task: &Task = &incomplete[usize::from(*item_number) - 1];
        match task_repo.delete_task(task.id).await {
            Ok(_) => println!("Removed task \"{}\". 🗑️", task.description),
            Err(err) => println!("Failed to remove task \"{}\". Error: {}", item_number, err),
        }
    }

    Ok(())
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
pub async fn mark_complete(task_repo: &mut impl TaskRepository, numbers: Vec<u16>) -> Result<()> {
    let unique = remove_duplicates(numbers);

    let incomplete = task_repo.incomplete_tasks().await?;

    println!();
    if incomplete.is_empty() {
        println!("There are no tasks to complete. 🤷");
        return Ok(());
    }

    for item_number in unique.iter() {
        if usize::from(*item_number) > incomplete.len() || item_number < &1 {
            println!("Invalid task number: {} 🤕", item_number,);
            continue;
        }

        let existing: &Task = &incomplete[usize::from(*item_number) - 1];
        match task_repo.mark_complete(existing.id).await {
            Ok(task) => println!("Marked task \"{}\" as completed. ✅", task.description),
            Err(err) => println!(
                "Failed to mark task \"{}\" as completed. Error: {}",
                item_number, err
            ),
        }
    }

    Ok(())
}

/// `get_completed_tasks` gets a list of all tasks which have no
/// completion date.
pub async fn get_completed_tasks(task_repo: &mut impl TaskRepository) -> Result<()> {
    let tasks = task_repo.completed_tasks().await?;

    println!();

    if tasks.is_empty() {
        println!("You have no completed tasks! Why not do some work? 🔨🪚");
        return Ok(());
    }

    println!("🎉 You have completed the following tasks:");
    tasks.into_iter().enumerate().for_each(|(idx, task)| {
        println!(
            "  {}. {} (completed: {})",
            idx + 1,
            task.description,
            task.complete_date
                .map(|dt| dt
                    .with_timezone(&Local)
                    .format("%m/%d/%Y @ %I:%M%p")
                    .to_string())
                .unwrap_or("".to_string())
        )
    });

    Ok(())
}

/// `get_incomplete_tasks` gets a list of all tasks which have a completion date.
pub async fn get_incomplete_tasks(task_repo: &mut impl TaskRepository) -> Result<()> {
    let tasks = task_repo.incomplete_tasks().await?;

    println!();

    if tasks.is_empty() {
        println!("You have no more tasks! Why not take a vacation? 🏖️");
        return Ok(());
    }

    println!("You have the following tasks:");
    tasks
        .into_iter()
        .enumerate()
        .for_each(|(idx, task)| println!("  {}. {}", idx + 1, task.description));

    Ok(())
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
