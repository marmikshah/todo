use std::{error, io};

use log::{debug, info, warn};

use crate::{config::Config, db::store::Store};

pub fn delete_task(id: &i32) -> Result<(), ()> {
    // info!("Deleting Task: {}", id);
    warn!("This is a destructive operation. Do you wish to proceed? (y/n)");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read");

    let input = input.trim();
    if input.to_lowercase() == "y" {
        debug!("Attempting to delete task: {}", id);
        let config = Config::new();
        let store = Store::new(&config.dbpath).unwrap();
        store.delete_record(id)
    } else {
        info!("Not deleting.");
        Ok(())
    }
}
