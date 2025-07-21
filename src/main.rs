use std::{cmp::min, fmt::format, io, process::Command, thread, time::Duration};
use ratatui::{
    backend::{Backend, CrosstermBackend}, layout::{Constraint, Direction, Layout}, style::{Color, Style}, text, widgets::{Block, BorderType, Borders, Paragraph, Widget}, Frame, Terminal
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

fn format_float(f: f64) -> String {
    if f > 1e4 {
        format!("{:.3E}", f)
    } else if f < 1e-2 {
        format!("{:.3E}", f)
    } else {
        format!("{:.3}", f)
    }
}

fn ui(f: &mut Frame<>, state: &GameState) {
    let mut area = f.area().clone();
    area.height = 12;

    let [score_row, other_row] = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref()
        )
        .split(area)[..2] else { todo!() };

    let score_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Max(20),   
                Constraint::Max(20),   
                Constraint::Max(20),   
                Constraint::Max(10),   
            ].as_ref()
        )
        .split(score_row);

    let score_block = Block::default()
        .title("Score")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let idle_increase_block = Block::default()
        .title("Idle increase")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let active_increase_block = Block::default()
        .title("Active increase")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    f.render_widget(Paragraph::new(format_float(state.score)).block(score_block), score_chunks[0]);
    f.render_widget(Paragraph::new(format_float(state.idle_increase)).block(idle_increase_block), score_chunks[1]);
    f.render_widget(Paragraph::new(format_float(state.active_increase)).block(active_increase_block), score_chunks[2]);

}

fn handle_events(state: &mut GameState) -> std::io::Result<()> {
    let timeout = Duration::from_secs_f32(1.0 / 100.0);
    if !event::poll(timeout)? {
        return Ok(());
    }
    if let Some(key) = event::read()?.as_key_press_event() {
        match key.code {
            KeyCode::Char(' ') | KeyCode::Enter => state.score += state.active_increase,
            _ => {}
        }
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    let mut state = GameState{ score: 0.0, active_increase: 1.0, idle_increase: 0.001 };
    
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let _ = Command::new("clear").spawn();


    loop {
        let result = terminal.draw(|f| ui(f, &state));
        if result.is_err() { break; }
        
        state.update();
        let _ = handle_events(&mut state);
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
