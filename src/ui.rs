
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Style, Stylize}, widgets::{Block, BorderType, Borders, List, ListDirection, Paragraph}, Frame};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

use crate::game::State;



pub struct Ui {
    score_chunks: Vec<Rect>,
    other_row: Rect,
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

        Ui { score_chunks, other_row, current_area: frame.area() }
    }

    pub fn update(&mut self, frame: &mut Frame, state: State) {
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

        let click_block = Block::default()
            .title("Enter")
            .borders(Borders::ALL)
            .border_type(BorderType::Double).border_type(BorderType::Rounded);

        frame.render_widget(Paragraph::new(format_float(*state.get_score())).block(score_block), self.score_chunks[0]);
        frame.render_widget(Paragraph::new(format_float(*state.get_idle_increase())).block(idle_increase_block), self.score_chunks[1]);
        frame.render_widget(Paragraph::new(format_float(*state.get_active_increase())).block(active_increase_block), self.score_chunks[2]);

        let list = List::new(["i", "j"])
            .block(click_block)
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::BottomToTop);

        frame.render_widget(list, self.other_row);

    }

}