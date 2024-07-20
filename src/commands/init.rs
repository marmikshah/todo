use std::io::Error;

use log::{debug, info, warn};

use crate::{config::Config, db::store::Store};

pub fn init() -> Result<(), Error> {
    let config = Config::new();
    if config.get_setup_status(true).is_ok() {
        // TODO: Check if table exists.
        warn!("Cannot re-initialize.");
        return Ok(());
    }

    if config.setup().is_ok() {
        debug!("Paths have been setup");
    }

    debug!("Initialising tasks table");

    let query = "
            CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                status INTEGER NOT NULL DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
        ";

    debug!("Attempting to initialise table in DB");
    debug!("Query: {}", query);

    let store = Store::new(&config.dbpath).unwrap();
    let result = store.query(&query).unwrap();
    debug!("{}", result);

    if result == 0 {
        info!("Successful!");
        debug!("Task table created successfully");
    }

    Ok(())
}
