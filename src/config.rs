use env_logger::Env;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::{env, fs};

use log::{debug, info, Level, LevelFilter};

#[derive(Debug)]
pub struct Config {
    pub path: String,
}

impl Default for Config {
    fn default() -> Self {
        log::set_max_level(LevelFilter::Debug);
        debug!("Initialising db @ ./");

        let path = env::var("TODO_DATASTORE_DIR")
                            .unwrap_or_else(|_| String::from("./")) + ".todo/";

        let result = fs::create_dir_all(&path);
        if result.is_ok() {
            debug!("Directory @ {} created successfully", &path);
        } else {
            panic!("Cannot create data storage directory. Do you have the correct write permissions?");
        }

        let loglevel = env::var("TODO_LOG_LEVEL").unwrap_or_else(|_| String::from("debug"));

        env_logger::Builder::from_env(Env::default().default_filter_or(&loglevel)).init();
        Config { path }
    }
}

pub static APP_CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::default()));
