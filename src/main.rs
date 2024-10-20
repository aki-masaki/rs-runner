mod app;
mod tasks_reader;
mod ui;

use crate::app::App;
use crate::event::Event;
use crate::ui::ui;
use ratatui::prelude::Backend;
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;

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
    }

    Ok(())
}
