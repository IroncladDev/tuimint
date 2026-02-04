use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Debug, Clone, Copy)]
pub enum Screen {
    Splash,
    Tutorial,
    Join,
    Wallets,
    Settings,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub screen: Screen,
    pub keys_typed: HashSet<KeyCode>,
    pub modifiers_held: KeyModifiers,
    pub focused: bool,
    pub quit: bool,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            screen: Screen::Splash,
            keys_typed: HashSet::new(),
            modifiers_held: KeyModifiers::empty(),
            focused: true,
            quit: false,
        }
    }

    pub fn lock(state: &Arc<Mutex<AppState>>) -> MutexGuard<'_, AppState> {
        state.lock().unwrap_or_else(|e| panic!("{}", e))
    }
}

// Mutable methods
impl AppState {
    /// Navigate to a new screen
    pub fn navigate(&mut self, screen: Screen) -> &mut Self {
        self.screen = screen;
        self
    }

    /// Insert a key into the set of keys typed
    pub fn insert_key_typed(&mut self, key: KeyCode) -> &mut Self {
        self.keys_typed.insert(key);
        self
    }

    /// Clear the set of keys typed
    pub fn clear_keys_typed(&mut self) -> &mut Self {
        self.keys_typed.clear();
        self
    }

    /// Set the key modifiers
    pub fn set_key_modifiers(&mut self, modifiers: KeyModifiers) -> &mut Self {
        self.modifiers_held = modifiers;
        self
    }

    /// Clear the key modifiers
    pub fn set_focus(&mut self, focused: bool) -> &mut Self {
        self.focused = focused;
        self
    }
}

impl Default for AppState {
    fn default() -> Self {
        AppState::new()
    }
}
