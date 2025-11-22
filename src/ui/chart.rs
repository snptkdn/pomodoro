use crate::app::state::App;
use crate::domain::pomodoro::PomodoroPhase;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 1)].as_ref())
        .split(size);

    let (phase_name, phase_color) = match app.pomodoro.phase {
        PomodoroPhase::Work => ("Work", Color::Red),
        PomodoroPhase::ShortBreak => ("Break", Color::Cyan),
        PomodoroPhase::LongBreak => ("Lunch", Color::Yellow),
    };

    let x_labels = vec![
        Span::styled(
            format!("{}", app.signal.x),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("Pomodoro - {}", phase_name)),
        Span::styled(
            format!(
                "{:0>2}:{:0>2}",
                (app.pomodoro.timer / 60.0).floor(),
                (app.pomodoro.timer % 60.0).floor()
            ),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ];

    let datasets = vec![
        Dataset::default()
            .name(phase_name)
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(phase_color))
            .data(&app.data),
        Dataset::default()
            .name("Cycle End")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Green))
            .data(&app.cycle_markers),
    ];

    let chart = Chart::new(datasets)
        .block(Block::default().borders(Borders::NONE))
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .labels(x_labels)
                .bounds(app.window),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([-20.0, 20.0]),
        );
    f.render_widget(chart, chunks[0]);
}
