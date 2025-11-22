
mod application;
mod domain;
mod event;
mod infrastructure;
mod presentation;

use crate::{
    application::app::App,
    event::event_loop::run_app,
    infrastructure::terminal::{CrosstermTerminalHandler, TerminalHandler},
};
use std::{error::Error, io, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    let mut handler = CrosstermTerminalHandler::new(io::stdout());
    let res = handler.run(|terminal| {
        let tick_rate = Duration::from_millis(1000);
        let app = App::new();
        run_app(terminal, app, tick_rate)
    });

    if let Err(err) = res {
        // Since the terminal is restored by the handler, we can safely print the error.
        println!("Error: {:?}", err);
        return Err(Box::new(err));
    }

    Ok(())
}
