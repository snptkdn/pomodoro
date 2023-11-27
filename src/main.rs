use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset},
    Frame, Terminal,
};

#[derive(Clone)]
pub struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    pub fn new(interval: f64, period: f64, scale: f64) -> SinSignal {
        SinSignal {
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    // fn next(&mut self) -> Option<Self::Item> {
    //     let point =  if self.x < 0.0 {
    //         (self.x, 0.0)
    //     } else {
    //         (self.x, (((self.x * 1.0 / self.period))).sin() * self.scale)
    //     };
    //     self.x += self.interval;
    //     Some(point)
    // }
    fn next(&mut self) -> Option<Self::Item> {
        let adjusted_x = self.x - 3600.0; // x から 3600 を減算して調整
        let point = if self.x < 0.0 {
            (self.x, 0.0)
        } else {
            (self.x, (adjusted_x / self.period).sin() * self.scale)
        };
        self.x += self.interval;
        Some(point)
    }
}

struct App {
    signal1: SinSignal,
    data1: Vec<(f64, f64)>,
    signal2: SinSignal,
    data2: Vec<(f64, f64)>,
    window: [f64; 2],
}

impl App {
    fn new() -> App {
        let mut signal1 = SinSignal::new(1.0, 37.5*5.0, 18.0); // 25min
        let mut signal2 = SinSignal::new(1.0, 37.5, 10.0); // 30min
        let data1 = signal1.by_ref().take(3600).collect::<Vec<(f64, f64)>>();
        let data2 = signal2.by_ref().take(3600).collect::<Vec<(f64, f64)>>();
        App {
            signal1,
            data1,
            signal2,
            data2,
            window: [0.0, 3600.0],
        }
    }

    fn on_tick(&mut self) {
        for _ in 0..1 {
            self.data1.remove(0);
        }
        self.data1.extend(self.signal1.by_ref().take(1));
        for _ in 0..1 {
            self.data2.remove(0);
        }
        self.data2.extend(self.signal2.by_ref().take(1));
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(1000);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::<B>(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 1),
            ]
            .as_ref(),
        )
        .split(size);
    let x_labels = vec![
        Span::styled(
            format!("{}", app.signal1.x),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("Pomodoro")),
        Span::styled(
            format!("{:0>2}:{}", (app.window[1]%1800.0/60.0).floor(), (app.window[1]%60.0).floor()),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ];
    let datasets = vec![
        Dataset::default()
            .name("data2")
            .marker(symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&app.data1),
        Dataset::default()
            .name("data3")
            .marker(symbols::Marker::Braille)
            .style(Style::default().fg(Color::Yellow))
            .data(&app.data2),
    ];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
        )
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
