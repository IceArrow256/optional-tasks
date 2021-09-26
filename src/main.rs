use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Task {
    id: u16,
    description: String,
}

#[derive(Serialize, Deserialize)]
struct Tasks {
    data: Vec<Task>,
}

impl Tasks {
    fn new() -> Tasks {
        let data: Vec<Task> = Vec::new();
        Tasks { data }
    }

    #[allow(dead_code)]
    fn add(&mut self, id: u16, description: String) {
        self.data.push(Task { id, description });
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

fn main() {
    #[allow(unused_mut)]
    let mut tasks = load_tasks();
    println!("{}", tasks);
    save_task(tasks);
}

fn get_tasks_file_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "IceArrow256", "optional-tasks").unwrap();
    let data_dir = proj_dirs.data_dir();
    data_dir.join("tasks.json")
}

fn load_tasks() -> Tasks {
    let tasks_file_path = get_tasks_file_path();
    if tasks_file_path.exists() {
        let mut data = String::new();
        let mut file = File::open(tasks_file_path).unwrap();
        file.read_to_string(&mut data)
            .expect("Unable to read string");
        serde_json::from_str(&data).ok().unwrap()
    } else {
        Tasks::new()
    }
}

fn save_task(tasks: Tasks) {
    let tasks_file_path = get_tasks_file_path();
    std::fs::create_dir(tasks_file_path.parent().unwrap()).ok();
    std::fs::write(tasks_file_path, serde_json::to_string(&tasks).ok().unwrap()).ok();
}
