mod screens;

use crate::{
    FRAME_RATE,
    message::Message,
    state::{AppState, Screen},
};
use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyModifiers, poll, read};
use ratatui::Frame;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::mpsc::UnboundedSender;

pub use screens::*;

/// Handles and delegates keyboard and mouse events for different screens
pub fn handle_events(state: &Arc<Mutex<AppState>>, tx: UnboundedSender<Message>) -> Result<()> {
    if poll(Duration::from_millis(1000 / FRAME_RATE))?
        && let Event::Key(key) = read()?
    {
        let screen = { AppState::lock(state)?.screen };
        match screen {
            Screen::Splash => SplashScreen::handle_keys(key, state, tx)?,
            Screen::Join => JoinScreen::handle_keys(key, state, tx)?,
            Screen::Wallets => WalletsScreen::handle_keys(key, state, tx)?,
            Screen::Settings => SettingsScreen::handle_keys(key, state, tx)?,
        }
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            AppState::lock(state)?.quit();
        }
    }
    Ok(())
}

/// Renders the UI
pub fn render(frame: &mut Frame, state: &Arc<Mutex<AppState>>) {
    let screen = state.lock().unwrap().screen;
    match screen {
        Screen::Splash => SplashScreen::render(frame, state),
        Screen::Join => JoinScreen::render(frame, state),
        Screen::Wallets => WalletsScreen::render(frame, state),
        Screen::Settings => SettingsScreen::render(frame, state),
    }
}
