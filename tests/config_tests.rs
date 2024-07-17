use std::char::DecodeUtf16Error;
use std::env;

use rusqlite::ffi::sqlite3_result_null;
use tempfile::tempdir;
use todo::config::{Config, ENV_VAR_DATASTORE_DIR};

fn set_env_var(key: &str, value: &str) {
    env::set_var(key, value);
}

fn clear_env_var(key: &str) {
    env::remove_var(key);
}

#[test]
fn test_default_config() {
    let _config = Config::new();
}

#[test]
fn test_is_setup_without_setup() {
    let temp_dir = tempdir().unwrap();
    set_env_var(&ENV_VAR_DATASTORE_DIR, temp_dir.path().to_str().unwrap());

    let config = Config::new();

    let result = config.get_setup_status();
    assert!(result.is_err() || !result.is_ok());
    clear_env_var(&ENV_VAR_DATASTORE_DIR);
}

#[test]
fn test_setup() {
    let temp_dir = tempdir().unwrap();
    set_env_var(&ENV_VAR_DATASTORE_DIR, temp_dir.path().to_str().unwrap());

    let mut config = Config::new();

    let status = config.get_setup_status();
    assert!(status.is_err() || !status.is_ok());

    let _ = config.setup();

    let status = config.get_setup_status();
    match &status {
        Ok(status) => println!("Setup OK"),
        Err(e) => println!("Setup not OK"),
    }
    assert!(status.is_ok());

    clear_env_var(&ENV_VAR_DATASTORE_DIR);
}
