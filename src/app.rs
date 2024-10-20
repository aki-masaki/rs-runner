use crate::event::KeyCode;
use crate::tasks_reader::{read_file, read_tasks, Task};
use crate::Event;

use ratatui::crossterm::event;
use std::io;

#[derive(Default)]
pub struct App {
    pub tasks: Vec<Task>,
    pub should_quit: bool,
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
                _ => {}
            }
        }

        Ok(())
    }
}
