mod screens;

use crate::{message::Message, state::{AppState, Screen}};
use ratatui::Frame;
use tokio::sync::mpsc::UnboundedSender;
use std::sync::{Arc, Mutex};

pub use screens::*;

/// Renders the UI
pub fn render(frame: &mut Frame, state: &Arc<Mutex<AppState>>, tx: UnboundedSender<Message>) {
    let screen = state.lock().unwrap().screen;
    match screen {
        Screen::Splash => SplashScreen::render(frame, state, tx),
        Screen::Join => JoinScreen::render(frame, state, tx),
        Screen::Wallets => WalletsScreen::render(frame, state, tx),
        Screen::Settings => SettingsScreen::render(frame, state, tx),
    }
}
