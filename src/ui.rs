use std::time::Duration;

use crossterm::event::{self, KeyCode};
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, widgets::{Block, BorderType, Borders, Paragraph}, Frame};

use crate::GameState;


pub struct Ui {
    score_chunks: Vec<Rect>,
    other_row: Rect,
    click_down: bool,
    current_area: Rect,
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

fn get_rows(frame: &Frame) -> (Vec<Rect>, Rect) {
    let mut area = frame.area().clone();
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

    let score_chunks: Vec<Rect> = Layout::default()
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
        .split(score_row).to_vec();

    (score_chunks, other_row)
}



impl Ui {

    pub fn default(frame: &Frame) -> Ui {
        let (score_chunks, other_row) = get_rows(frame);

        Ui { score_chunks, other_row, click_down: false, current_area: frame.area() }
    }

    pub fn update(&mut self, frame: &mut Frame, state: &GameState) {
        let area = frame.area().clone();
        if area.width != self.current_area.width || area.height != self.current_area.height {
            self.current_area = area;
            (self.score_chunks, self.other_row) = get_rows(frame);
        }

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

        frame.render_widget(Paragraph::new(format_float(state.score)).block(score_block), self.score_chunks[0]);
        frame.render_widget(Paragraph::new(format_float(state.idle_increase)).block(idle_increase_block), self.score_chunks[1]);
        frame.render_widget(Paragraph::new(format_float(state.active_increase)).block(active_increase_block), self.score_chunks[2]);


    }

    pub fn handle_events(&self, state: &mut GameState) -> std::io::Result<()> {
        let timeout = Duration::from_secs_f32(1.0 / 5.0);
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
}