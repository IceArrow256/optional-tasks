use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::env;
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

    /// add task to tasks
    fn add(&mut self, description: String) {
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
    fn del(&mut self, id: u16) {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tasks = load_tasks();
    if args.len() > 1 {
        match args[1].as_ref() {
            "add" => {
                if args.len() > 2 {
                    let description = args[2..].join(" ");
                    tasks.add(description.to_owned());
                }
            }
            _ => {
                let id: Option<u16> = match args[1].parse() {
                    Ok(num) => Some(num),
                    Err(_) => None,
                };
                if let Some(id) = id {
                    match args[2].as_ref() {
                        "del" => tasks.del(id),
                        _ => {}
                    }
                }
            }
        }
    }
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
