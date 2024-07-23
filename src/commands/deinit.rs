use std::{fs, io};

use log::{debug, info};

use crate::config::Config;

pub fn deinit() -> Result<(), ()> {
    info!("This is a destructive operation. You will not be able to revert this.");
    info!("Do you wish to proceed? (y/n)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");

    let input = input.trim();
    if input.to_lowercase() == "y" {
        debug!("Attempting to delete everything");
        let config = Config::default();
        match fs::remove_file(config.dbpath) {
            Ok(_) => {
                info!("Removed database successfully");
                Ok(())
            }
            Err(e) => {
                debug!("Deletion error {}", e);
                Err(())
            }
        }
    } else {
        info!("Not deleting.");
        Ok(())
    }
}
