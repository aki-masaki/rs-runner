use crate::tasks_reader::TaskState;
use crate::App;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Stylize;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::text::Text;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;
use ratatui::Frame;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(frame.area());

    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(main_chunks[1]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded);

    let tasks_title = Paragraph::new(Text::styled(
        "Tasks",
        Style::default().fg(Color::Green).bold(),
    ))
    .block(block.clone());

    let output_title = Paragraph::new(Text::styled(
        "Output",
        Style::default().fg(Color::Green).bold(),
    ))
    .block(block.clone());

    let help_title = Paragraph::new(Text::styled(
        "Help",
        Style::default().fg(Color::Green).bold(),
    ))
    .block(block.clone());

    let inspect_title = Paragraph::new(Text::styled(
        "Inspect",
        Style::default().fg(Color::Green).bold(),
    ))
    .block(block);

    if app.is_help_open {
        frame.render_widget(help_title, main_chunks[0]);
    } else {
        frame.render_widget(output_title, main_chunks[0]);
    }

    frame.render_widget(tasks_title, sidebar_chunks[0]);
    frame.render_widget(inspect_title, sidebar_chunks[1]);

    let mut tasks_area = sidebar_chunks[0];
    tasks_area.y += 3;
    tasks_area.x += 3;

    let mut inspect_area = sidebar_chunks[1];
    inspect_area.y += 3;
    inspect_area.x += 1;

    let mut output_area = main_chunks[0];
    output_area.y += 3;
    output_area.x += 1;

    frame.render_widget(Text::from(render_tasks(app)), tasks_area);

    if app.is_help_open {
        frame.render_widget(Text::from(render_help(app)), output_area);
    } else {
        frame.render_widget(
            Paragraph::new(app.output.clone()).wrap(Wrap { trim: true }),
            output_area,
        );
    }

    frame.render_widget(Text::from(render_inspector(app)), inspect_area);
}

fn render_tasks(app: &mut App) -> Vec<Line> {
    let mut lines: Vec<Line> = vec![];
    let mut line: Line;
    let mut content: Vec<Span>;

    for (i, task) in app.tasks.iter().enumerate() {
        content = vec![
            Span::from(task.name.clone()),
            match task.state {
                TaskState::Started => Span::styled(" started", Style::default().fg(Color::Green)),
                TaskState::Stopped => Span::styled(" stopped", Style::default().fg(Color::Magenta)),
                TaskState::Finished => Span::styled(" finished", Style::default().fg(Color::Blue)),
                TaskState::Error => Span::styled(" error", Style::default().fg(Color::Red)),
            },
        ];

        if i == app.selected_index {
            line = Line::styled(
                "",
                Style::default()
                    .fg(match task.state {
                        TaskState::Started => Color::Green,
                        TaskState::Stopped => Color::Magenta,
                        TaskState::Finished => Color::Blue,
                        TaskState::Error => Color::Red,
                    })
                    .italic(),
            )
            .spans(content);
        } else {
            line = Line::from(content);
        }

        lines.push(line);
    }

    lines
}

fn render_inspector(app: &mut App) -> Vec<Line> {
    let task = app.tasks[app.selected_index].clone();

    vec![
        Line::from("").spans(vec![
            Span::styled("Name: ", Style::default().fg(Color::Blue)),
            Span::styled(task.name, Style::default().fg(Color::Magenta)),
        ]),
        Line::from("").spans(vec![
            Span::styled("Command: ", Style::default().fg(Color::Blue)),
            Span::styled(task.command, Style::default().fg(Color::Magenta)),
        ]),
        Line::from("").spans(vec![
            Span::styled("Dir: ", Style::default().fg(Color::Blue)),
            Span::styled(task.dir, Style::default().fg(Color::Magenta)),
        ]),
        Line::from("").spans(vec![Span::styled(
            "Args: ",
            Style::default().fg(Color::Blue),
        )]),
        Line::from("").spans(vec![Span::styled(
            "   ".to_string() + task.args.join(" ").as_str(),
            Style::default().fg(Color::Magenta),
        )]),
    ]
}

fn render_help(_app: &mut App) -> Vec<Line> {
    vec![
        Line::from("").spans(vec![
            Span::styled("q ", Color::Blue),
            Span::styled("quit", Color::Magenta),
        ]),
        Line::from("").spans(vec![
            Span::styled("h ", Color::Blue),
            Span::styled("help", Color::Magenta),
        ]),
        Line::from("").spans(vec![
            Span::styled("j ", Color::Blue),
            Span::styled("down", Color::Magenta),
        ]),
        Line::from("").spans(vec![
            Span::styled("k ", Color::Blue),
            Span::styled("up", Color::Magenta),
        ]),
        Line::from("").spans(vec![
            Span::styled("s ", Color::Blue),
            Span::styled("run", Color::Magenta),
        ]),
        Line::from("").spans(vec![
            Span::styled("r ", Color::Blue),
            Span::styled("refresh", Color::Magenta),
        ]),
    ]
}
