use crate::state::Screen;
use crate::ui::prelude::*;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub struct SettingsScreen {}

impl Component for SettingsScreen {
    fn render(&mut self, frame: &mut Frame, state: &AppStateMutex) {
        let state = state.lock().unwrap();

        if state.screen != Screen::Settings {
            return;
        }

        let paragraph = ratatui::widgets::Paragraph::new("Settings");
        let block = Block::bordered().border_style(Style::new().green().on_white().bold().italic());
        frame.render_widget(paragraph.clone().block(block), frame.area());
    }
}
