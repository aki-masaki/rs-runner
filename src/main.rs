mod app;
mod tasks_reader;
mod ui;

use crate::app::App;
use crate::event::Event;
use crate::tasks_reader::TaskState;
use crate::ui::ui;
use ratatui::prelude::Backend;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;
use std::sync::Arc;

use ratatui::crossterm::event;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let _ = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), io::Error> {
    loop {
        if app.should_quit {
            break;
        }

        terminal.draw(|x| ui(x, app))?;

        app.handle_events()?;

        if app.rx.is_none() {
            continue;
        }

        let rx = Arc::clone(&app.rx).clone();

        if let Some(receiver) = rx.as_ref() {
            if let Ok(result) = receiver.try_recv() {
                let output = result.1?;

                if !output.stderr.is_empty() {
                    app.output = String::from_utf8(output.stderr).unwrap();
                    app.tasks[app.selected_index].state = TaskState::Error;

                    continue;
                }

                let index = app
                    .tasks
                    .iter()
                    .position(|x| x.name == result.0)
                    .unwrap_or(app.selected_index);

                let output_string = String::from_utf8(output.stdout).unwrap();

                app.tasks[index].output = Some(output_string.clone());

                if index == app.selected_index {
                    app.output = output_string;
                }

                app.tasks[index].state = TaskState::Finished;
            }
        }
    }

    Ok(())
}
