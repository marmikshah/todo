use std::fs;
use std::path::PathBuf;

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
    #[error("Failed to create directory: {0}")]
    CreateDirError(#[from] std::io::Error),
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

        debug!("Application Directory: {}", &app_dir.display());
        debug!("Database Path: {}", &db_path.display());

        let config = Config {
            path: app_dir,
            dbpath: db_path,
        };

        config
    }

    pub fn get_setup_status(&self, checkdb: bool) -> Result<(), ()> {
        if self.path.exists() {
            if checkdb {
                if self.dbpath.exists() {
                    return Ok(());
                }
                return Err(());
            }
            return Ok(());
        }

        Err(())
    }

    pub fn setup(&self) -> Result<(), ConfigError> {
        if self.get_setup_status(false).is_ok() {
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
