use std::sync::mpsc::Receiver;

use std::thread::sleep;
use std::time::Duration;
use std::{io, process::Command};

use ratatui::widgets::ListState;

use crate::{control::Action, ui::Ui};

#[derive(Clone)]
pub struct State {
    score: f64,
    active_increase: f64, 
    idle_increase: f64,
    list_state: ListState,
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

    pub fn get_list_state(&mut self) -> &mut ListState {
        &mut self.list_state
    } 
}

pub struct Game {
    state: State,
    control_rx: Receiver<Action>,
    ui: Option<Ui>,
}

impl Game {
    pub fn default(control_rx: Receiver<Action>) -> Game {
        Game {
            state: State { score: 0.0, active_increase: 1.0, idle_increase: 0.0, list_state: ListState::default() },
            control_rx,
            ui: None,
        }
    }

    pub fn run(mut self) -> Result<(), io::Error> {
        let mut terminal = ratatui::init();
        let _ = Command::new("clear").spawn();
        self.state.list_state.select_first();


        loop {
            if !self.update() { break; };
    
            let val = &mut self.ui;
    
            if let Err(_) = terminal.draw(|f| {
                if val.is_none() {
                    *val = Some(Ui::default(f)); 
                }

                val.as_mut().unwrap().update(f, &mut self.state);    
            }) { break; }
        
            sleep(Duration::from_millis(100));
                        
        }

        let _ = Command::new("clear").spawn();

        ratatui::restore();

        Ok(())
    }

    fn update(&mut self) -> bool {
        self.state.update();

        while let Ok(action) = self.control_rx.try_recv() { 
            match action {
                Action::ArrowUp => self.state.list_state.select_previous(),
                Action::ArrowDown => self.state.list_state.select_next(),
                Action::Deselect => todo!(),
                Action::Exit => return false,
                Action::SelectUpgrade => todo!(),
                Action::Click => self.state.score += self.state.active_increase,
            }
        }

        return true;
    }
}