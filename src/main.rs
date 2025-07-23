mod ui;
mod game;

use std::{io, process::Command, time::Duration};
use ratatui::{
    backend::CrosstermBackend, Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use crate::{game::Game, ui::Ui};


fn main() -> Result<(), io::Error> {


    Ok(()) 
}
