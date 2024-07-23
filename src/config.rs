use std::fs;
use std::path::PathBuf;

use log::{debug, error, warn, LevelFilter};

pub const ENV_VAR_DATASTORE_DIR: &str = "TODO_DATASTORE_DIR";
pub const PROJECT_NAME: &str = "todo";

#[derive(Debug)]
pub struct Config {
    pub path: PathBuf,
    pub dbpath: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to create directory: {0}")]
    CreateDirError(#[from] std::io::Error),
    #[error("Cannot find app directory. Has the app been initialised?")]
    AppDirError,
    #[error("Cannot find database file")]
    DatabaseFileError,
}

impl Config {
    pub fn get_setup_status(&self, checkdb: bool) -> Result<(), ConfigError> {
        if !self.path.exists() {
            return Err(ConfigError::AppDirError);
        }
        if !self.dbpath.exists() && checkdb {
            return Err(ConfigError::DatabaseFileError);
        }

        Ok(())
    }

    pub fn setup(&self) -> Result<(), ConfigError> {
        if self.get_setup_status(false).is_ok() {
            warn!("All paths have already been setup. Running this again will have no effect");
            return Ok(());
        }
        if fs::create_dir(&self.path).is_err() {
            panic!(
                "Failed to create directory @ {}",
                (self.path).to_string_lossy()
            );
        }
        debug!("Directory created successfully");

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
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

        debug!(
            "Application Directory: {}, Exists: {}",
            &app_dir.display(),
            &app_dir.exists()
        );
        debug!(
            "Database Path: {}, Exists: {}",
            &db_path.display(),
            &db_path.exists()
        );

        Config {
            path: app_dir,
            dbpath: db_path,
        }
    }
}
