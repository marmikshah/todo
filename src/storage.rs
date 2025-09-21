//! File-based storage using TOML format.

use crate::model::*;
use colored::*;
use log::debug;
use std::collections::HashMap;
use std::path::PathBuf;

/// Storage system for persisting application data to TOML files.
pub(crate) struct Storage {
    /// Path to the TOML file where data is stored
    path: PathBuf,
    /// In-memory application data
    pub data: AppData,
}

impl Storage {
    /// Creates a new storage instance, loading data from file if it exists.
    ///
    /// If the file doesn't exist or can't be read, an empty data structure is created.
    pub fn new() -> Self {
        let path = Self::get_storage_path();

        let data = if path.exists() {
            if let Ok(file_content) = std::fs::read_to_string(path.clone()) {
                toml::from_str(&file_content).unwrap_or_else(|e| {
                    log::error!("Failed to load data from {}: {}", path.display(), e);
                    AppData {
                        tasks: HashMap::new(),
                    }
                })
            } else {
                AppData {
                    tasks: HashMap::new(),
                }
            }
        } else {
            AppData {
                tasks: HashMap::new(),
            }
        };

        debug!("Using Storage Path: {}", path.display());

        Self { path, data }
    }

    /// Determines the storage path based on build configuration.
    ///
    /// In debug mode, uses `.todo.toml` in the current directory.
    /// In release mode, uses `.todo.toml` in the user's home directory.
    fn get_storage_path() -> PathBuf {
        if cfg!(debug_assertions) {
            return PathBuf::from(".todo.toml");
        }
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("./"))
            .join(".todo.toml")
    }

    /// Saves the current data to the TOML file with user feedback.
    ///
    /// # Arguments
    ///
    /// * `message_ok` - Success message to display if save succeeds
    /// * `message_err` - Error message to display if save fails
    pub fn save(&self, message_ok: &str, message_err: &str) {
        if let Ok(content) = toml::to_string_pretty(&self.data) {
            if let Err(e) = std::fs::write(self.path.clone(), content) {
                println!("Failed to update local storage: {}", e.to_string().red());
                println!("{}", message_err.red());
            } else {
                println!("{}", message_ok.green());
            }
        } else {
            let message = "Unable to update local storage, Parsing Failed!";
            println!("{}", message.red());
        }
    }
}
