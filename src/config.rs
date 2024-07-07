use std::io::Error;
use std::path::Path;
use std::{env, fs};

use log::{debug, info, warn, LevelFilter};

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub dbpath: String,
    pub statuspath: String,
    pub is_initialised: bool,
}

impl Config {
    pub fn update_init_status(self) -> Result<(), Error> {
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        log::set_max_level(LevelFilter::Debug);
        debug!("Initialising db @ ./");

        let path = env::var("TODO_DATASTORE_DIR").unwrap_or_else(|_| String::from("./")) + ".todo/";

        let result = fs::create_dir_all(&path);
        if result.is_ok() {
            debug!("Directory @ {} created successfully", &path);
        } else {
            panic!(
                "Cannot create data storage directory. Do you have the correct write permissions?"
            );
        }

        let mut is_initialised = false;
        let status_file_path = &format!("{}.status", path);
        let path_ref = Path::new(&status_file_path);

        if !path_ref.exists() {
            is_initialised = false;
            debug!("Store is not initialised.");
            warn!("Run `todo init` to initialise storage");
        } else {
        }

        let dbpath = String::from(&path) + "/.db";

        let statuspath = String::from(&path) + "/.status";
        Config {
            path,
            dbpath,
            statuspath,
            is_initialised,
        }
    }
}
