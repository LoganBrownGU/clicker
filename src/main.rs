use std::{process::Command, thread, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal,
    Frame
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

struct GameState {
    score: f64, 
    idle_increase: f64,
    active_increase: f64,
}

impl GameState {
    fn update(&mut self) {
        self.score += self.idle_increase; 
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
   let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());
    let block = Block::default()
         .title("Block")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default()
         .title("Block 2")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}

fn main() -> Result<(), io::Error> {
    let mut state = GameState{ score: 0.0, active_increase: 1.0, idle_increase: 1.0 };
    
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    Command::new("clear").spawn();

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

    terminal.draw(|f| ui(f));

    thread::sleep(Duration::from_millis(5000));
    
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Command::new("clear").spawn();

    Ok(()) 
}
