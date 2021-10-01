use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
struct Task {
    id: u16,
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    data: Vec<Task>,
}

impl Tasks {
    pub fn new() -> Tasks {
        let data: Vec<Task> = Vec::new();
        Tasks { data }
    }

    /// add task to tasks
    pub fn add(&mut self, description: String) {
        let mut id = 0u16;
        for task in &self.data {
            if task.id > id {
                id = if task.id > id { task.id } else { id };
            }
        }
        self.data.push(Task {
            id: id + 1,
            description,
        });
    }
    pub fn del(&mut self, id: u16) {
        match self.data.iter().position(|task| task.id == id) {
            Some(position) => {
                self.data.remove(position);
            }
            _ => {
                println!("No tasks specified.");
            }
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.description)
    }
}

impl fmt::Display for Tasks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for task in &self.data {
            output.push_str(&format!("{}\n", task));
        }
        write!(f, "{}", output)
    }
}
