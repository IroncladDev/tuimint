use std::sync::{Arc, Mutex};

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{style::Style, widgets::Block};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    message::Message,
    state::{AppState, Screen},
    ui::UIScreen,
};

pub struct SplashScreen {}

impl UIScreen for SplashScreen {
    fn render(frame: &mut ratatui::Frame, state: &Arc<Mutex<AppState>>) {
        let mut state = state.lock().unwrap();
        state.increment();
        let text = format!("Count: {} {:?}", state.count, state.screen);
        let paragraph = ratatui::widgets::Paragraph::new(text);
        let block = Block::bordered().border_style(Style::new().blue().on_white().bold().italic());
        frame.render_widget(paragraph.clone().block(block), frame.area());
    }

    fn handle_keys(
        ev: KeyEvent,
        state: &Arc<Mutex<AppState>>,
        _tx: UnboundedSender<Message>,
    ) -> Result<()> {
        let mut state = AppState::lock(state)?;

        if ev.code == KeyCode::Char('s') {
            state.navigate(Screen::Settings)
        } else if ev.code == KeyCode::Char('w') {
            state.navigate(Screen::Wallets)
        } else if ev.code == KeyCode::Char('j') {
            state.navigate(Screen::Join)
        }

        Ok(())
    }
}
