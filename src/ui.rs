use crate::App;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Stylize;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::text::Text;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(frame.area());

    let mut lines: Vec<Line> = vec![];
    let mut line: Line;

    for (i, task) in app.tasks.iter().enumerate() {
        if i == app.selected_index {
            line = Line::styled(task.name.clone(), Style::default().fg(Color::Cyan).italic());
        } else {
            line = Line::from(task.name.clone());
        }

        lines.push(line);
    }

    let sidebar_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default());

    let sidebar_title = Paragraph::new(Text::styled(
        "Tasks",
        Style::default().fg(Color::Green).bold(),
    ))
    .block(sidebar_block);

    let output_block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default());

    let output_title = Paragraph::new(Text::styled(
        "Output",
        Style::default().fg(Color::Green).bold(),
    ))
    .block(output_block);

    frame.render_widget(output_title, chunks[0]);
    frame.render_widget(sidebar_title, chunks[1]);

    let mut sidebar_area = chunks[1];
    sidebar_area.y += 3;
    sidebar_area.x += 3;

    let mut output_area = chunks[0];
    output_area.y += 3;
    output_area.x += 1;

    frame.render_widget(Text::from(lines), sidebar_area);
    frame.render_widget(Text::from(app.output.clone()), output_area);
}
