use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Store {
    connection: Connection,
}

impl Store {
    /// Creates a new instance of Store class.
    /// /// Creates a new instance of Store class
    pub fn new(path: &str) -> Self {
        println!("Establishing connection");

        let conn = Connection::open("todo.db").unwrap();
        Store { connection: conn }
    }

    // Private function that will create the database if it doesn't exist.
    pub fn init_db(&self) -> Result<()> {

        let query = "
            CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                status INTEGER NOT NULL DEFAULT 0,
                due_date DATETIME,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
        ";

        let result = self.connection.execute(&query, ())?;
        println!("Result: {}", result);

        Ok(())
    }
}