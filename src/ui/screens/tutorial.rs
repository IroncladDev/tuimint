use std::sync::{Arc, Mutex};

use ratatui::{
    layout::{Alignment, Constraint},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph},
};

use crate::{
    state::{AppState, Screen},
    ui::Component,
};

pub struct TutorialScreen {}

impl Component for TutorialScreen {
    fn render(&mut self, frame: &mut ratatui::Frame, state: &Arc<Mutex<AppState>>) {
        let state = state.lock().unwrap();

        if state.screen != Screen::Tutorial {
            return;
        }

        let lines = vec![Line::from("Tutorial")];

        let text = Text::from(lines).style(Style::default().fg(Color::Blue));
        let paragraph = Paragraph::new(text).alignment(Alignment::Center);

        let block = Block::bordered()
            .border_style(Style::new().blue())
            .padding(Padding::vertical(1));
        frame.render_widget(
            paragraph.clone().block(block),
            frame
                .area()
                .centered(Constraint::Max(60), Constraint::Max(18)),
        );
    }
}
