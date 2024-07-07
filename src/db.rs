use log::{debug, info, warn};
use rusqlite::{Connection, Params, Result};

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
}
