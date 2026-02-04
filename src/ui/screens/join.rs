use std::sync::{Arc, Mutex};

use ratatui::{style::Style, widgets::Block};

use crate::{
    state::{AppState, Screen},
    ui::Component,
};

pub struct JoinScreen {}

impl Component for JoinScreen {
    fn draw(&self, frame: &mut ratatui::Frame, state: &Arc<Mutex<AppState>>) {
        let state = state.lock().unwrap();

        if state.screen != Screen::Join {
            return;
        }

        let paragraph = ratatui::widgets::Paragraph::new("Join");
        let block = Block::bordered().border_style(Style::new().red().on_white().bold().italic());
        frame.render_widget(paragraph.clone().block(block), frame.area());
    }
}
