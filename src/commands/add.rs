use log::{debug, error};

use crate::{config::Config, db::store::Store};

pub fn add_task(item: &str, store: &Store) {
    debug!("Adding task: {}", item);

    match store.add_task(item) {
        Ok(_) => {
            debug!("Task added successfully")
        }
        Err(_) => {
            error!("Failed to add task")
        }
    }
}
