#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub count: u64,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            screen: Screen::Splash,
            count: 0,
        }
    }
}

// Mutable methods
impl AppState {
    /// Navigate to a new screen
    pub fn navigate(&mut self, screen: Screen) -> &mut Self {
        self.screen = screen;
        self
    }
}
