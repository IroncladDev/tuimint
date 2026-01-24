use color_eyre::{Result, eyre::eyre};
use crossterm::event::{KeyCode, KeyModifiers};
use std::{collections::HashSet, sync::{Arc, Mutex, MutexGuard}};

#[derive(Debug, Clone, Copy)]
pub enum Screen {
    Splash,
    Join,
    Wallets,
    Settings,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub count: i32,
    pub screen: Screen,
    /// Tracks keys typed by the user
    /// Typed keys appear in this vec for a single frame
    pub keys_typed: HashSet<KeyCode>,
    pub modifiers_held: KeyModifiers,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            count: 0,
            screen: Screen::Splash,
            keys_typed: HashSet::new(),
            modifiers_held: KeyModifiers::empty(),
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
}
