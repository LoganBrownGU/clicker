mod ui;

use std::{io, process::Command, time::Duration};
use ratatui::{
    backend::CrosstermBackend, Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};
use crate::ui::Ui;

struct GameState {
    score: f64, 
    idle_increase: f64,
    active_increase: f64,
}

impl GameState {
    fn update(&mut self, do_active_increase: bool) {
        self.score += self.idle_increase; 
        if do_active_increase { self.score += self.active_increase }
    }
}


fn main() -> Result<(), io::Error> {
    let mut state = GameState{ score: 0.0, active_increase: 1.0, idle_increase: 0.001 };
    
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let _ = Command::new("clear").spawn();

    let mut ui: Option<Ui> = None;
    loop {
        let val = &mut ui;
        let result = terminal.draw(|f| {
            if val.is_some() {
                val.as_mut().unwrap().update(f, &state);    
            } else {
                *val = Some(Ui::default(f)); val.as_mut().unwrap().update(f, &state);
            }    
        });
        if result.is_err() { break; }
        
        let _ = val.as_mut().unwrap().handle_events();
        state.update(val.as_mut().unwrap().was_click_performed());
    }   

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    let _ = Command::new("clear").spawn();

    Ok(()) 
}
