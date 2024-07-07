use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

use log::{debug, info};

use crate::{db::Store, config::Config};

pub fn initialise_tables(config: &mut Config) -> Result<(), Error> {
    debug!("Initialising tasks table");
    let store = Store::new().unwrap();

    debug!("Connected to table");

    let query = "
            CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                status INTEGER NOT NULL DEFAULT 0,
                due_date DATETIME,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
        ";

    let result = store.query(&query).unwrap();
    debug!("{}", result);

    let status_file_path = String::from(&config.path) + ".inited";

    if result == 0 {
        info!("Successful!");
        debug!("Task table created successfully");
        config.is_initialised = true;
    } else {
        panic!("Failed to initialise store.");
    }

    Ok(())
}
