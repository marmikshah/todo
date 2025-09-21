use crate::{
    model::task::{TaskItem, TaskPriority, TaskStatus},
    storage::Storage,
    utils::parse_date,
};
use uuid::Uuid;

/// Creates a new task with the provided description and optional parameters.
///
/// The task will be assigned a unique UUID and default values for status and priority
/// if not specified. If no due date is provided, it defaults to end of today.
///
/// # Arguments
///
/// * `storage` - Mutable reference to the storage system
/// * `key` - Unique key for the task (must not already exist)
/// * `description` - Human-readable task description
/// * `priority` - Optional priority (defaults to `medium`)
/// * `status` - Optional status (defaults to `not-started`)
/// * `due` - Optional due date in YYYYMMDD format
///
/// # Errors
///
/// This function will return early if:
/// * The key already exists
/// * The due date format is invalid
/// * The due date is in the past
pub fn handle_add(
    storage: &mut Storage,
    key: String,
    description: String,
    priority: Option<TaskPriority>,
    status: Option<TaskStatus>,
    due: Option<String>,
) {
    let mut task = TaskItem {
        id: Uuid::new_v4(),
        ..Default::default()
    };

    if storage.data.tasks.contains_key(&key) {
        println!("Task with key {} already exists", key);
        return;
    }

    task.description = description.clone();
    if let Some(priority) = priority {
        task.priority = priority;
    }
    if let Some(status) = status {
        task.status = status;
    }
    if let Some(due) = due {
        if let Ok(date) = parse_date(&due) {
            task.due_date = date;
        } else {
            println!("Invalid due date format. Please use YYYYMMDD format");
            return;
        }
    }

    storage.data.tasks.insert(key.clone(), task);
    storage.save("✓ Task added successfully!", "Failed to add task");
}
