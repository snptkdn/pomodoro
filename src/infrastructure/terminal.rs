
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::{Backend, CrosstermBackend}, Terminal};
use std::io;

pub trait TerminalHandler<'a> {
    type B: Backend;
    fn run<F>(&'a mut self, app_logic: F) -> Result<(), io::Error>
    where
        F: FnOnce(&mut Terminal<Self::B>) -> Result<(), io::Error>;
}

pub struct CrosstermTerminalHandler<W: io::Write> {
    writer: W,
}

impl<W: io::Write> CrosstermTerminalHandler<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }
}

impl<'a, W: io::Write + 'a> TerminalHandler<'a> for CrosstermTerminalHandler<W> {
    type B = CrosstermBackend<&'a mut W>;

    fn run<F>(&'a mut self, app_logic: F) -> Result<(), io::Error>
    where
        F: FnOnce(&mut Terminal<Self::B>) -> Result<(), io::Error>,
    {
        // Setup terminal
        enable_raw_mode()?;
        execute!(&mut self.writer, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(&mut self.writer);
        let mut terminal = Terminal::new(backend)?;

        // Run application logic
        let res = app_logic(&mut terminal);

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        res
    }
}
