
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Chart, Dataset, GraphType},
    Frame,
};

use crate::application::app::App;

pub fn ui<B: Backend>(f: &mut Frame, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(1, 1)].as_ref())
        .split(size);
    let x_labels = vec![
        Span::styled(
            format!("{}", app.signal1.x()),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("Pomodoro")),
        Span::styled(
            format!(
                "{:0>2}:{}",
                (app.window[1] % 1800.0 / 60.0).floor(),
                (app.window[1] % 60.0).floor()
            ),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ];
    let sin_datasets = vec![
        Dataset::default()
            .name("Break")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Cyan))
            .data(&app.data1),
        Dataset::default()
            .name("Work")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Red))
            .data(&app.data2),
        Dataset::default()
            .name("Lunch")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .data(&app.data3),
    ];

    let periods = [
        (app.signal1.period(), Color::Cyan),
        (app.signal2.period(), Color::Red),
        (app.signal3.period(), Color::Yellow),
    ];
    let vertical_lines_data =
        generate_vertical_lines_data(app.window, app.y_bounds, &periods);
    let line_datasets: Vec<Dataset> = vertical_lines_data
        .iter()
        .map(|(data, color)| {
            Dataset::default()
                .graph_type(GraphType::Line)
                .style(Style::default().fg(*color).add_modifier(Modifier::BOLD))
                .data(data)
        })
        .collect();

    let datasets = [sin_datasets, line_datasets].concat();

    let chart = Chart::new(datasets)
        .block(Block::default())
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .labels(x_labels)
                .bounds(app.window),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds(app.y_bounds),
        );
    f.render_widget(chart, chunks[0]);
}

fn generate_vertical_lines_data(
    window: [f64; 2],
    y_bounds: [f64; 2],
    periods: &[(f64, Color)],
) -> Vec<(Vec<(f64, f64)>, Color)> {
    let mut vertical_lines_data_with_color = Vec::new();
    for (period, color) in periods.iter() {
        if *period == 0.0 {
            continue;
        }
        let start_k = (window[0] / period).ceil() as i32;
        let end_k = (window[1] / period).floor() as i32;
        for k in start_k..=end_k {
            let x = k as f64 * period;
            vertical_lines_data_with_color.push((vec![(x, y_bounds[0]), (x, y_bounds[1])], *color));
        }
    }
    vertical_lines_data_with_color
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_vertical_lines_data() {
        let window = [0.0, 1000.0];
        let y_bounds = [-10.0, 10.0];
        let periods = [(300.0, Color::Cyan), (600.0, Color::Red), (900.0, Color::Yellow)];
        let data = generate_vertical_lines_data(window, y_bounds, &periods);
        assert_eq!(data.len(), 4 + 2 + 2); // 4 for signal1, 2 for signal2, 2 for signal3
    }
}
