use log::{debug, error};

use crate::{config::Config, db::Store};

pub fn add_task(item: &str, config: &Config) {
    debug!("Adding task: {}", item);

    if !config.is_initialised {
        error!("todo is not initialised. Please run `todo init`");
    }

    let store = Store::new().unwrap();

    match store.add_task(item) {
        Ok(_) => {
            debug!("Task added successfully")
        }
        Err(_) => {
            error!("Failed to add task")
        }
    }
}
