use std::sync::{Arc, Mutex};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{style::Style, widgets::Block};
use tokio::sync::mpsc::UnboundedSender;
use color_eyre::Result;

use crate::{message::Message, state::{AppState, Screen}, ui::UIScreen};

pub struct WalletsScreen {}

impl UIScreen for WalletsScreen {
    fn render(frame: &mut ratatui::Frame, _state: &Arc<Mutex<AppState>>) {
        let paragraph = ratatui::widgets::Paragraph::new("Wallets");
        let block = Block::bordered().border_style(Style::new().cyan().on_white().bold().italic());
        frame.render_widget(paragraph.clone().block(block), frame.area());
    }

    fn handle_keys(ev: KeyEvent, state: &Arc<Mutex<AppState>>, _tx: UnboundedSender<Message>) -> Result<()> {
        let mut state = AppState::lock(state)?;
        if ev.code == KeyCode::Char('s') {
            state.navigate(Screen::Settings)
        } else if ev.code == KeyCode::Char('h') {
            state.navigate(Screen::Splash)
        } else if ev.code == KeyCode::Char('j') {
            state.navigate(Screen::Join)
        }

        Ok(())
    }
}
