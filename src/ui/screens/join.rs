use crate::state::Screen;
use crate::ui::prelude::*;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub struct JoinScreen {}

impl Component for JoinScreen {
    fn draw(&self, frame: &mut Frame, state: &AppStateMutex) {
        let state = state.lock().unwrap();

        if state.screen != Screen::Join {
            return;
        }

        let paragraph = ratatui::widgets::Paragraph::new("Join");
        let block = Block::bordered().border_style(Style::new().red().on_white().bold().italic());
        frame.render_widget(paragraph.clone().block(block), frame.area());
    }
}
