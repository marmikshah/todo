use log::{debug, error};
use todo::config::Config;

use crate::db::store::Store;

pub fn add_task(item: &str) {
    debug!("Adding task: {}", item);

    let config = Config::default();
    let store = Store::new(&config.dbpath).unwrap();

    match store.add_task(item) {
        Ok(_) => {
            debug!("Task added successfully")
        }
        Err(_) => {
            error!("Failed to add task")
        }
    }
}
