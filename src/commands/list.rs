use log::{self, debug};
use todo::config::Config;

use crate::db::store::Store;

pub fn list_tasks() {
    debug!("Requesting task list from db");

    let config = Config::new();
    let store = Store::new(&config.dbpath).unwrap();
    let result = store.get_tasks();

    match &result {
        Ok(tasks) => {
            for task in tasks {
                println!("{}", task);
            }
        }
        Err(_) => {
            panic!("Failed to retrieve tasks");
        }
    }
}
