use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
struct Task {
    id: u16,
    description: String,
}

fn main() {
    let proj_dirs = ProjectDirs::from("com", "IceArrow256", "optional-tasks").unwrap();
    let data_dir = proj_dirs.data_dir();
    std::fs::create_dir(data_dir).ok();
    let mut tasks: Vec<Task> = Vec::new();
    let paths = std::fs::read_dir(data_dir).unwrap();
    for path in paths {
        let mut data = String::new();
        let mut f = File::open(path.unwrap().path()).expect("Unable to open file");
        f.read_to_string(&mut data).expect("Unable to read string");
        tasks.push(serde_json::from_str(&data).ok().unwrap())
    }
    for task in tasks {
        println!("id: {}, description: {}", task.id, task.description);
        std::fs::write(
            data_dir.join(task.id.to_string() + ".json"),
            serde_json::to_string(&task).ok().unwrap(),
        )
        .ok();
    }
}
