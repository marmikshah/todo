use log::{debug, info, warn};
use rusqlite::{params, Connection, Params, Result, ToSql};

use crate::Config;

pub struct Store {
    connection: Connection,
}

impl Store {
    pub fn new() -> Result<Self> {
        let config = Config::default();

        debug!("Connecting to {}", &config.dbpath);

        let conn = Connection::open(&config.dbpath).unwrap();
        Ok(Store { connection: conn })
    }

    pub fn query(&self, stmt: &str) -> Result<usize> {
        self.connection.execute(stmt, ())
    }

    pub fn add_task(&self, item: &str) -> Result<usize, rusqlite::Error> {
        // This function assumes all checks are done.
        debug!("Preparing statment to add task");

        let ret = self
            .connection
            .prepare("INSERT INTO tasks (description) VALUES (?)");

        match ret {
            Ok(mut stmt) => stmt.execute(params![item]),
            Err(_) => {
                panic!("Failed to add task")
            }
        }
    }
}
