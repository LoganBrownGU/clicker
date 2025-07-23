use std::{sync::mpsc::Sender, time::Duration};

use crossterm::event::{self, KeyCode};


pub struct Control {
    tx: Sender<()>,
}

impl Control {
    pub fn default(tx: Sender<()>) -> Control {
        Control { tx }
    }
    
    pub fn run(self) {
        let mut click_depressed = false;

        loop {
            if !event::poll(Duration::from_millis(50)).unwrap_or(false) {
                continue;
            }

            let key_event = event::read();
            if key_event.is_err() { continue; }

            let is_click = match key_event.unwrap().as_key_press_event() {
                Some(key) => key.code == KeyCode::Char(' '),
                None => false,
            };

            
            eprintln!("{}", is_click);

            if  is_click && !click_depressed {
                click_depressed = true; 
                continue;
            }
            if !is_click &&  click_depressed {
                let _ = self.tx.send(());             
                eprintln!("{}", is_click);
                click_depressed = false; 
            }


        }
    } 
}