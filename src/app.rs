use crate::event::KeyCode;
use crate::tasks_reader::{read_file, read_tasks, Task};
use crate::Event;
use std::process::Command;

use ratatui::crossterm::event;
use std::io;

#[derive(Default)]
pub struct App {
    pub tasks: Vec<Task>,
    pub should_quit: bool,
    pub output: String,
    pub selected_index: usize,
}

impl App {
    pub fn new() -> App {
        App {
            tasks: read_tasks(read_file("/home/niki/tasks.toml".to_string())),
            ..Default::default()
        }
    }

    pub fn handle_events(&mut self) -> Result<(), io::Error> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    self.should_quit = !self.should_quit;
                }
                KeyCode::Char('j') => {
                    if self.selected_index < self.tasks.len() - 1 {
                        self.selected_index += 1;
                    }
                }
                KeyCode::Char('k') => {
                    if self.selected_index > 0 {
                        self.selected_index -= 1;
                    }
                }
                KeyCode::Char('s') => {
                    let _ = self.run_task();
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn run_task(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let task = self.tasks[self.selected_index].clone();

        self.tasks[self.selected_index].is_running = true;

        let output = Command::new(task.command).args(task.args).output()?;

        self.output = String::from_utf8(output.stdout)?;

        Ok(())
    }
}
