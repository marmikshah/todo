use crate::storage::Storage;
use colored::*;

/// Removes a task from storage using its unique key.
///
/// The task will be permanently deleted and the change will be saved.
///
/// # Arguments
///
/// * `storage` - Mutable reference to the storage system
/// * `key` - Unique key of the task to delete
///
/// # Errors
///
/// This function will return early if the task key is not found.
pub fn handle_delete(storage: &mut Storage, key: String) {
    println!("Deleting task: {}", key);

    if !storage.data.tasks.contains_key(&key) {
        println!("Task with key {} not found", key);
        return;
    }

    if storage.data.tasks.remove(&key).is_some() {
        storage.save("✓ Task deleted successfully!", "Failed to delete task");
    } else {
        println!("Task with key {} not found", key.red());
    }
}
