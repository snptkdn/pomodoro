
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};
use std::{
    io,
    time::{Duration, Instant},
};

use crate::{application::app::App, presentation::ui::ui};

pub fn run_app<B: Backend>(
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
            if app.window[1] == 3600.0 {
                app = App::new();
            }
            last_tick = Instant::now();
        }
    }
}
