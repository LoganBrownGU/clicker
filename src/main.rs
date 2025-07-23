mod ui;
mod game;
mod control;

use std::{io, process::Command, sync::mpsc::channel, thread, time::Duration};
use ratatui::{
    backend::CrosstermBackend, Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use crate::{control::Control, game::Game, ui::Ui};


fn main() -> Result<(), io::Error> {

    let (tx, rx) = channel();

    let game = Game::default(rx);
    let control = Control::default(tx);

    let _ = thread::spawn(|| {
        control.run();
    });

    let _ = game.run();

    Ok(()) 
}
