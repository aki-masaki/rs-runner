use std::fs;

#[derive(Debug)]
pub struct Task {
    name: String,
    command: String,
    args: Vec<String>,
}

pub fn read_file(path: String) -> String {
    fs::read_to_string(path).unwrap_or("".to_string())
}

pub fn read_tasks(content: String) -> Vec<Task> {
    let map = content.parse::<toml::Table>().unwrap();

    let mut tasks: Vec<Task> = vec![];

    for task in map.into_iter() {
        tasks.push(Task {
            name: task.1.get("name").unwrap().to_string(),
            command: task.1.get("command").unwrap().to_string(),
            args: task
                .1
                .get("args")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.to_string())
                .collect(),
        })
    }

    tasks
}