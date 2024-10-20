use ratatui::text::Line;
use ratatui::text::Text;
use ratatui::Frame;
use crate::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let mut lines: Vec<Line> = vec![];

    for task in &app.tasks {
        lines.push(Line::from(task.name.clone()));
    }

    frame.render_widget(Text::from(lines), frame.area());
}
