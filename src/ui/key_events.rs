use std::sync::{Arc, Mutex};

use crossterm::event::{Event, KeyCode};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    message::Message,
    state::{AppState, Screen},
    ui::{JoinScreen, SettingsScreen, SplashScreen, UIScreen, WalletsScreen},
};
use color_eyre::Result;

/// Handles key events sent from the UI
/// Updates state or sends messages to the backend
pub fn handle_event(
    ev: Event,
    state: &Arc<Mutex<AppState>>,
    tx: UnboundedSender<Message>,
) -> Result<()> {
    if let Event::Key(key) = ev {
        let screen = { AppState::lock(state)?.screen };
        match screen {
            Screen::Splash => SplashScreen::handle_keys(key, state, tx)?,
            Screen::Join => JoinScreen::handle_keys(key, state, tx)?,
            Screen::Wallets => WalletsScreen::handle_keys(key, state, tx)?,
            Screen::Settings => SettingsScreen::handle_keys(key, state, tx)?,
        }
        if key.code == KeyCode::Char('q') {
            AppState::lock(state)?.quit();
        }
    }
    Ok(())
}
