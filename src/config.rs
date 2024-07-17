use std::path::PathBuf;
use std::{fmt::format, fs};

use log::{debug, error, warn, LevelFilter};

pub const ENV_VAR_DATASTORE_DIR: &str = "TODO_DATASTORE_DIR";
pub const PROJECT_NAME: &str = "todo";

#[derive(Debug)]
pub struct Config {
    path: PathBuf,
    pub dbpath: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to determine App directory: {0}")]
    AppDirError(String),
    #[error("Failed to create directory: {0}")]
    CreateDirError(#[from] std::io::Error),
    #[error("Cannot reassign path ")]
    ReassignPathError(String),
}

impl Config {
    /// Return a new instance of Config with all nones.
    pub fn new() -> Self {
        log::set_max_level(LevelFilter::Debug);

        let ret = std::env::var(ENV_VAR_DATASTORE_DIR);
        let mut app_dir: PathBuf;
        match ret {
            Ok(path) => app_dir = PathBuf::from(path),
            Err(_) => app_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("./")),
        }

        app_dir.push(&format!(".{}", PROJECT_NAME));

        let mut db_path: PathBuf = PathBuf::from(&app_dir);
        db_path.push("app.db");

        let config = Config {
            path: app_dir,
            dbpath: db_path,
        };

        if config.get_setup_status().is_err() {
            debug!(
                "Setup has never been run before. Please run {} init",
                PROJECT_NAME
            );
        }

        config
    }

    pub fn get_setup_status(&self) -> Result<(), ()> {
        if !self.path.exists() {
            warn!("Application directory has not been setup");
            return Err(());
        }

        if !self.dbpath.exists() {
            warn!("Database file not found.");
            return Err(());
        }

        if self.path.exists() && self.dbpath.exists() {
            return Ok(());
        }

        Err(())
    }

    pub fn setup(&self) -> Result<(), ConfigError> {
        if self.get_setup_status().is_ok() {
            warn!("All paths have already been setup. Running this again will have no effect");
            return Ok(());
        }
        if fs::create_dir(&self.path).is_err() {
            panic!(
                "Failed to create directory @ {}",
                (&self.path).to_string_lossy()
            );
        }
        debug!("Directory created successfully");

        Ok(())
    }
}
