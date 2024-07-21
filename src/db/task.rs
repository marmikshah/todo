use std::fmt;

use log::info;

#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub description: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t\t{}\t\t", self.id, self.description)
    }
}

impl Task {
    pub fn print_headers() {
        info!("Task ID\t\tTask Name");
    }
}
