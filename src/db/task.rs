use std::fmt;

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
