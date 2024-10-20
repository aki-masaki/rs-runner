mod tasks_reader;

use crate::tasks_reader::{read_file, read_tasks};

fn main() {
    let tasks = read_tasks(read_file("/home/niki/tasks.toml".to_string()));

    println!("{:?}", tasks[0]);
}
