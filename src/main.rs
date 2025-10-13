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
use log::trace;
use log4rs::{append::{console::{ConsoleAppender, Target}, file::FileAppender}, config::{Appender, Root}, Config};
use crate::{control::Control, game::Game, ui::Ui};


fn main() -> Result<(), io::Error> {

    let file_logger = FileAppender::builder().append(true).build("./log").unwrap();

    let root = Root::builder()
        .appender("fl")
        .build(log::LevelFilter::Trace);
    let config = Config::builder()
        .appender(Appender::builder().build("fl", Box::new(file_logger)))
        .build(root)
        .unwrap();

    let _ = log4rs::init_config(config);

    let (tx, rx) = channel();

    let game = Game::default(rx);
    let control = Control::default(tx);

    let _ = thread::spawn(|| {
        control.run();
    });

    let _ = game.run();

    Ok(()) 
}
