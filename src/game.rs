use std::sync::mpsc::Receiver;

use std::thread::sleep;
use std::time::Duration;
use std::{io, process::Command};

use log::trace;
use rand::Rng;
use ratatui::widgets::ListState;
use strum::EnumCount;
use strum_macros::{EnumCount, FromRepr};

use crate::{control::Action, ui::Ui};

#[derive(Clone, FromRepr, EnumCount, Debug, PartialEq)]
enum UpgradeType {
    Active,
    Idle
}

impl UpgradeType {
    pub fn random() -> UpgradeType {
        let mut rng = rand::rng();
        UpgradeType::from_repr(rng.random_range(0..UpgradeType::COUNT)).unwrap()
    }
}

impl ToString for UpgradeType {
    fn to_string(&self) -> String {
        match self {
            UpgradeType::Active => "active",
            UpgradeType::Idle => "idle",
        }.to_string()
    }
}

#[derive(Clone,Debug)]
pub struct Upgrade {
    upgrade_type: UpgradeType,
    pub level: f64,
    pub cost: f64,
}

impl Upgrade { 
    pub fn random(state: Option<&State>) -> Upgrade {
        let mut rng = rand::rng();
        let level = rng.random_range(0.0..10.0);

        let min_cost = match state {
            Some(s) => 10f64.powf(s.score.log10().floor() + 1.0),
            None => 0.0,
        };
        let max_cost = match state {
            Some(s) => 10f64.powf(s.score.log10().floor() + 2.0),   
            None => 10.0,
        };
        let cost = rng.random_range(min_cost..=max_cost);

        Upgrade { upgrade_type: UpgradeType::random(), level, cost }
    }
}

impl ToString for Upgrade {
    fn to_string(&self) -> String {
        format!(" +{}; level: {:.2}; costs: {:.2}", self.upgrade_type.to_string(), self.level, self.cost)
    }
}

#[derive(Clone,Debug)]
pub struct State {
    score: f64,
    active_increase: f64, 
    idle_increase: f64,
    list_state: ListState,
    upgrade_list: Vec<Upgrade>,
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

    pub fn get_upgrade_list(&mut self) -> &mut Vec<Upgrade> {
        &mut self.upgrade_list
    } 

    pub fn adjust_score(&mut self, d: f64) {
        self.score += d;
    }

    pub fn increase_active_increase(&mut self, d: f64) {
        self.active_increase += d;
    }
    
    pub fn increase_idle_increase(&mut self, d: f64) {
        self.idle_increase += d;
    }
}

pub struct Game {
    state: State,
    control_rx: Receiver<Action>,
    ui: Option<Ui>,
}

impl Game {
    fn handle_upgrade(&mut self) {
        let upgrade = match self.state.get_list_state().selected() {
            Some(s) => self.state.get_upgrade_list().remove(s),
            None => todo!(),
        };

        if self.state.get_score() - upgrade.cost < 0 as f64 {
            self.state.get_upgrade_list().push(upgrade);
            return;
        }

        match upgrade.upgrade_type {
            UpgradeType::Active => self.state.increase_active_increase(upgrade.level),
            UpgradeType::Idle => self.state.increase_idle_increase(upgrade.level),
        }

        self.state.adjust_score(-upgrade.cost);

        let state_clone = self.state.clone();
        trace!("Upgrade");
        self.state.get_upgrade_list().push(Upgrade::random(Some(&state_clone)));
    }

    pub fn default(control_rx: Receiver<Action>) -> Game {
        let upgrade_list = (0..3).map(|_| Upgrade::random(None)).collect();

        Game {
            state: State { score: 0.0, active_increase: 1.0, idle_increase: 0.0, list_state: ListState::default(), upgrade_list },
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
        
            sleep(Duration::from_millis(10));
                        
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
                Action::SelectUpgrade => self.handle_upgrade(),
                Action::Click => self.state.score += self.state.active_increase,
            }
        }

        return true;
    }
}
