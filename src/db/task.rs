#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub status: bool,
}

pub struct TaskParamLengths {
    pub id: usize,
    pub description: usize,
    pub status: usize,
}

impl Task {
    pub fn status_to_string(&self) -> String {
        if self.status {
            String::from("Completed")
        } else {
            String::from("In Progress")
        }
    }
}
