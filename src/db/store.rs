use std::path::PathBuf;

use log::{debug, info, warn};
use rusqlite::{params, Connection, Params, Result, ToSql};

use super::task::Task;

pub struct Store {
    connection: Connection,
}

impl Store {
    pub fn new(dbpath: &PathBuf) -> Result<Self> {
        debug!("Connecting to {}", dbpath.display());

        let conn = Connection::open(dbpath).unwrap();
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

    pub fn get_tasks(&self) -> Result<Vec<Task>, rusqlite::Error> {
        let query = " SELECT * FROM tasks; ";

        let mut stmt = self.connection.prepare(&query)?;

        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                description: row.get(1)?,
            })
        })?;

        let mut tasks: Vec<Task> = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }

        Ok(tasks)
    }
}
