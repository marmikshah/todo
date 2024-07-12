use std::{
    fs,
    fs::File,
    io::{BufRead, BufReader, Error},
};

use log::{debug, info, warn};

use crate::{config::Config, db::store::Store};

pub fn init(config: &mut Config) -> Result<(), Error> {
    debug!("Initialising tasks table");
    let store = Store::new().unwrap();

    debug!("Connected to table");

    let query = "
            CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                status INTEGER NOT NULL DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
        ";

    let status_file_path = String::from(&config.path) + ".inited";

    match fs::metadata(&status_file_path) {
        Ok(_) => {
            config.is_initialised = true;
        }
        Err(_) => {
            debug!(".inited not found");
        }
    }

    if !config.is_initialised {
        debug!("Attempting to initialise table in DB");
        debug!("Query: {}", query);
        let result = store.query(&query).unwrap();
        debug!("{}", result);

        if result == 0 {
            info!("Successful!");
            debug!("Task table created successfully");
            match File::create(&status_file_path) {
                Ok(_) => {
                    debug!("Created init file successfully")
                }
                Err(_) => {
                    panic!("Failed to create init file")
                }
            }
            config.is_initialised = true;
        } else {
            panic!("Failed to initialise store.");
        }
    }

    Ok(())
}
