use std::sync::{Arc, Mutex};

use ratatui::Frame;

use crate::{
    state::{AppState, Screen},
    ui::{JoinScreen, SettingsScreen, SplashScreen, UIScreen, WalletsScreen},
};

/// The main render function for the UI
pub fn render(frame: &mut Frame, state: &Arc<Mutex<AppState>>) {
    let screen = state.lock().unwrap().screen;
    match screen {
        Screen::Splash => SplashScreen::render(frame, state),
        Screen::Join => JoinScreen::render(frame, state),
        Screen::Wallets => WalletsScreen::render(frame, state),
        Screen::Settings => SettingsScreen::render(frame, state),
    }
}
