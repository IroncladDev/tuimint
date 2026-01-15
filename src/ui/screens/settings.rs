use std::sync::{Arc, Mutex};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{style::Style, widgets::Block};
use tokio::sync::mpsc::UnboundedSender;

use crate::{message::Message, state::{AppState, Screen}, ui::UIScreen};
use color_eyre::Result;

pub struct SettingsScreen {}

impl UIScreen for SettingsScreen {
    fn render(frame: &mut ratatui::Frame, _state: &Arc<Mutex<AppState>>) {
        let paragraph = ratatui::widgets::Paragraph::new("Settings");
        let block = Block::bordered().border_style(Style::new().green().on_white().bold().italic());
        frame.render_widget(paragraph.clone().block(block), frame.area());
    }

    fn handle_keys(ev: KeyEvent, state: &Arc<Mutex<AppState>>, _tx: UnboundedSender<Message>) -> Result<()> {
        let mut state = AppState::lock(state)?;
        if ev.code == KeyCode::Char('h') {
            state.navigate(Screen::Splash)
        } else if ev.code == KeyCode::Char('w') {
            state.navigate(Screen::Wallets)
        } else if ev.code == KeyCode::Char('j') {
            state.navigate(Screen::Join)
        }

        Ok(())
    }
}
