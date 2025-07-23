use std::io::Stdout;
use std::sync::mpsc::Receiver;

use std::thread::sleep;
use std::time::Duration;
use std::{io, process::Command};
use ratatui::crossterm::event;
use ratatui::CompletedFrame;
use ratatui::{
    backend::CrosstermBackend, Terminal
};
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};

use crate::ui::Ui;

#[derive(Clone)]
pub struct State {
    score: f64,
    active_increase: f64, 
    idle_increase: f64,
}

impl State {
    fn update(&mut self) {
        self.score += self.idle_increase; 
    }

    pub fn get_score(&self) -> &f64 {
        &self.score
    } 
    
    pub fn get_active_increase(&self) -> &f64 {
        &self.active_increase
    }
    
    pub fn get_idle_increase(&self) -> &f64 {
        &self.idle_increase
    }  
}

pub struct Game {
    state: State, 
    control_rx: Receiver<()>,
    ui: Option<Ui>,
}

impl Game {
    pub fn default(control_rx: Receiver<()>) -> Game {
        Game {
            state: State { score: 0.0, active_increase: 1.0, idle_increase: 0.0 },
            control_rx,
            ui: None,
        }
    }

    pub fn run(mut self) -> Result<(), io::Error> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        
        //let _ = Command::new("clear").spawn();

        loop {
            self.update();
    
            let val = &mut self.ui;
    
            if let Err(_) = terminal.draw(|f| {
                if val.is_none() {
                    *val = Some(Ui::default(f)); 
                }

                val.as_mut().unwrap().update(f, self.state.clone());    
            }) { break; }
        
            sleep(Duration::from_millis(100));
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

    fn update(&mut self) {
        self.state.update();

        while let Ok(()) = self.control_rx.try_recv() {  
            self.state.score += self.state.active_increase;
        }
    }
}