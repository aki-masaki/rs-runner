use std::fs;

#[derive(Debug, Clone)]
pub enum TaskState {
    Stopped,
    Started,
    Finished,
    Error
}

#[derive(Debug, Clone)]
pub struct Task {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub state: TaskState,
    pub dir: String,
    pub output: Option<String>
}

pub fn read_file(path: String) -> String {
    fs::read_to_string(path).unwrap_or("".to_string())
}

pub fn read_tasks(content: String) -> Vec<Task> {
    let map = content.parse::<toml::Table>().unwrap();

    let mut tasks: Vec<Task> = vec![];

    for task in map.into_iter() {
        tasks.push(Task {
            name: task.1.get("name").unwrap().as_str().unwrap().to_string(),
            command: task.1.get("command").unwrap().as_str().unwrap().to_string(),
            args: task
                .1
                .get("args")
                .unwrap()
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect(),
            state: TaskState::Stopped,
            dir: task.1.get("dir").unwrap().as_str().unwrap().to_string(),
            output: None
        })
    }

    tasks
}
