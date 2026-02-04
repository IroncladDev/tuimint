use std::sync::{Arc, Mutex};

use ratatui::{style::Style, widgets::Block};

use crate::{
    state::{AppState, Screen},
    ui::Component,
};

pub struct SettingsScreen {}

impl Component for SettingsScreen {
    fn render(&self, frame: &mut ratatui::Frame, state: &Arc<Mutex<AppState>>) {
        let state = state.lock().unwrap();

        if state.screen != Screen::Settings {
            return;
        }

        let paragraph = ratatui::widgets::Paragraph::new("Settings");
        let block = Block::bordered().border_style(Style::new().green().on_white().bold().italic());
        frame.render_widget(paragraph.clone().block(block), frame.area());
    }
}
