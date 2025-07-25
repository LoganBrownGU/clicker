
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Style, Stylize}, widgets::{Block, BorderType, Borders, List, ListDirection, ListItem, ListState, Paragraph, StatefulWidget}, Frame};
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

        Ui { score_chunks, other_row, current_area: frame.area(), }
    }

    pub fn update(&mut self, frame: &mut Frame, state: &mut State) {
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

        let list = List::new([
                ListItem::from("item a"),
                ListItem::from("item b"),
            ])
            .block(click_block)
            .highlight_style(Style::new().italic())
            .highlight_symbol(">>")
            .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
            .repeat_highlight_symbol(true);

        
        frame.render_stateful_widget(list, self.other_row, state.get_list_state());

    }

}