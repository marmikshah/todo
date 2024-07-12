use log::{self, debug};

use crate::{config::Config, db::store::Store};

pub fn list_tasks(config: &Config) {
    debug!("Requesting task list from db");

    let store = Store::new().unwrap();
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
