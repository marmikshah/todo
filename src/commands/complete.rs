use crate::{model::task::TaskStatus, storage::Storage};

/// Marks a task as completed using its unique key.
///
/// The task's status will be changed to `Completed` and the change will be saved.
///
/// # Arguments
///
/// * `storage` - Mutable reference to the storage system
/// * `key` - Unique key of the task to complete
///
/// # Errors
///
/// This function will return early if the task key is not found.
pub fn handle_complete(storage: &mut Storage, key: String) {
    println!("Completing task: {}", key);

    if !storage.data.tasks.contains_key(&key) {
        println!("Task with key {} not found", key);
        return;
    }

    if let Some(task) = storage.data.tasks.get_mut(&key) {
        task.status = TaskStatus::Completed;
    }

    storage.save("✓ Task completed successfully!", "Failed to complete task");
}
