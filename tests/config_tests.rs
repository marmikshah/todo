use std::env;

use env_logger::Env;
use rusqlite::ffi::sqlite3_result_null;
use std::io::Write;
use tempfile::tempdir;
use todo::config::{Config, ENV_VAR_DATASTORE_DIR};

fn setup_logger() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();
}

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

    let result = config.get_setup_status(false);
    assert!(result.is_err() || !result.is_ok());
    clear_env_var(&ENV_VAR_DATASTORE_DIR);
}

#[test]
fn test_setup() {
    setup_logger();
    let temp_dir = tempdir().unwrap();
    set_env_var(&ENV_VAR_DATASTORE_DIR, temp_dir.path().to_str().unwrap());

    let config = Config::new();

    let status = config.get_setup_status(false);
    assert!(status.is_err() || !status.is_ok());

    let _ = config.setup();

    let status = config.get_setup_status(false);
    match &status {
        Ok(_) => println!("Setup OK"),
        Err(_) => println!("Setup not OK"),
    }
    assert!(status.is_ok());

    clear_env_var(&ENV_VAR_DATASTORE_DIR);
}
