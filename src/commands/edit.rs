use crate::{
    model::task::{TaskPriority, TaskStatus},
    storage::Storage,
    utils::parse_date,
};
use colored::Colorize;

/// Modifies an existing task's properties using its key.
///
/// Only the provided parameters will be updated. If a parameter is `None`,
/// the existing value is left unchanged.
///
/// # Arguments
///
/// * `storage` - Mutable reference to the storage system
/// * `key` - Unique key of the task to modify
/// * `description` - Optional new description
/// * `priority` - Optional new priority
/// * `status` - Optional new status
/// * `due` - Optional new due date in YYYYMMDD format
///
/// # Errors
///
/// This function will return early if:
/// * The task key is not found
/// * The due date format is invalid
/// * The due date is in the past
pub fn handle_edit(
    storage: &mut Storage,
    key: String,
    description: Option<String>,
    priority: Option<TaskPriority>,
    status: Option<TaskStatus>,
    due: Option<String>,
) {
    println!("Editing task: {}", key);

    // For now, just print the parameters
    if !storage.data.tasks.contains_key(&key) {
        println!("Task with key {} not found", key);
        return;
    }

    if let Some(task) = storage.data.tasks.get_mut(&key) {
        if let Some(description) = description {
            println!("New description: {}", description);
            task.description = description;
        }

        if let Some(priority) = priority {
            println!("New priority: {:?}", priority);
            task.priority = priority;
        }

        if let Some(status) = status {
            println!("New status: {:?}", status);
            task.status = status;
        }

        if let Some(due) = due {
            println!("New due date: {}", due);
            if let Ok(date) = parse_date(&due) {
                task.due_date = date;
            } else {
                println!("Invalid due date format. Please use YYYYMMDD format");
                return;
            }
        }
        storage.save(
            "✓ Task edited successfully!",
            "Failed to update local storage",
        );
    } else {
        println!("Task with key {} not found", key.red());
    }
}
