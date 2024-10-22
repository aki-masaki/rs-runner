use crate::event::KeyCode;
use crate::tasks_reader::{read_file, read_tasks, Task};
use crate::Event;
use std::io::Read;
use std::process::Child;
use std::process::Command;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

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

        let output = Command::new(task.command)
            .current_dir(task.dir)
            .args(task.args)
            .output();

        match output {
            Ok(output) => {
               self.output = String::from_utf8(output.stdout)?;
            }
            Err(ref e) => {
                self.output = e.to_string();
            }
        }

        Ok(())

        /*
        let output: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![]));
        let mut child_stdout = child.stdout.unwrap(); // Get the stdout

        let output_clone = Arc::clone(&output);
        thread::spawn(move || {
            let mut buffer = [0; 1024]; // Buffer for reading
            loop {
                match child_stdout.read(&mut buffer) {
                    Ok(0) => break, // EOF reached
                    Ok(n) => {
                        let mut data = output_clone.lock().unwrap();
                        data.extend_from_slice(&buffer[..n]);
                    }
                    Err(e) => {
                        eprintln!("Error reading stdout: {}", e);
                        break;
                    }
                }
            }
        });

        self.output = String::from_utf8(output.lock().unwrap().to_vec())
            .unwrap();

        Ok(())
        */
    }
}
