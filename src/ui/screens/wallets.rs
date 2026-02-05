use crate::state::Screen;
use crate::ui::prelude::*;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub struct WalletsScreen {}

impl Component for WalletsScreen {
    fn render(&mut self, frame: &mut Frame, state: &AppStateMutex) {
        let state = state.lock().unwrap();

        if state.screen != Screen::Wallets {
            return;
        }

        let paragraph = ratatui::widgets::Paragraph::new("Wallets");
        let block = Block::bordered().border_style(Style::new().cyan().on_white().bold().italic());
        frame.render_widget(paragraph.clone().block(block), frame.area());
    }
}
