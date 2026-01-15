use color_eyre::{Result, eyre::eyre};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug, Clone, Copy)]
pub enum Screen {
    Splash,
    Join,
    Wallets,
    Settings,
}

#[derive(Debug, Clone, Copy)]
pub struct AppState {
    pub count: i32,
    pub screen: Screen,
    pub should_quit: bool,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            count: 0,
            screen: Screen::Splash,
            should_quit: false,
        }
    }

    pub fn lock(state: &Arc<Mutex<AppState>>) -> Result<MutexGuard<'_, AppState>> {
        state.lock().map_err(|e| eyre!("{}", e))
    }

    pub fn navigate(&mut self, screen: Screen) {
        self.screen = screen;
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    pub fn decrement(&mut self) {
        self.count -= 1;
    }

    pub fn double(&mut self) {
        self.count *= 2;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
