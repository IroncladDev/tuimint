use std::sync::{Arc, Mutex};

use ratatui::{style::Style, widgets::Block};

use crate::{
    state::{AppState, Screen},
    ui::Component,
};

pub struct WalletsScreen {}

impl Component for WalletsScreen {
    fn render(&mut self, frame: &mut ratatui::Frame, state: &Arc<Mutex<AppState>>) {
        let state = state.lock().unwrap();

        if state.screen != Screen::Wallets {
            return;
        }

        let paragraph = ratatui::widgets::Paragraph::new("Wallets");
        let block = Block::bordered().border_style(Style::new().cyan().on_white().bold().italic());
        frame.render_widget(paragraph.clone().block(block), frame.area());
    }
}
