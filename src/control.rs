use std::{process::exit, sync::mpsc::Sender, time::Duration};

use ratatui::crossterm::event::{self, Event, KeyCode};

pub enum Action {
    ArrowUp,
    ArrowDown,
    Deselect,
    Exit,
    SelectUpgrade,
    Click,
}

pub struct Control {
    tx: Sender<Action>,
}

impl Control {
    pub fn default(tx: Sender<Action>) -> Control {
        Control { tx }
    }
    
    pub fn run(self) {
        let mut click_depressed = false;

        loop {
            if !event::poll(Duration::from_millis(50)).unwrap_or(false) {
                continue;
            }
            
            let mut is_click = false;
            if let Ok(Event::Key(key)) = event::read() {
                match key.code {
                    KeyCode::Char('f') => is_click = true,
                    KeyCode::Char('d') => is_click = false,
                    KeyCode::Up => self.tx.send(Action::ArrowUp).unwrap(),
                    KeyCode::Down => self.tx.send(Action::ArrowDown).unwrap(),
                    KeyCode::Esc => self.tx.send(Action::Deselect).unwrap(),
                    KeyCode::Char('q') => self.tx.send(Action::Exit).unwrap(),
                    _ => {},
                }
            }
            
            if  is_click && !click_depressed {
                click_depressed = true; 
                continue;
            }
            
            if !is_click &&  click_depressed {
                let _ = self.tx.send(Action::Click);             
                click_depressed = false; 
            }
        }
    } 
}