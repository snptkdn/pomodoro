
mod application;
mod domain;
mod event;
mod infrastructure;
mod presentation;

use crate::{
    application::app::App,
    event::event_loop::run_app,
    infrastructure::terminal::{restore_terminal, setup_terminal},
};
use std::{error::Error, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    let mut terminal = setup_terminal()?;

    // create app and run it
    let tick_rate = Duration::from_millis(1000);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    restore_terminal(&mut terminal)?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
