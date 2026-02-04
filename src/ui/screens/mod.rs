pub mod join;
pub mod settings;
pub mod splash;
pub mod tutorial;
pub mod wallets;

use std::sync::{Arc, Mutex};

use crate::{message::Message, state::AppState};
use anyhow::Result;
use crossterm::event::{Event, KeyEvent, MouseEvent};
use tokio::sync::mpsc::UnboundedSender;

pub trait UIScreen {
    fn render(
        frame: &mut ratatui::Frame,
        state: &Arc<Mutex<AppState>>,
        tx: UnboundedSender<Message>,
    );

    fn handle_event(event: Event, state: &Arc<Mutex<AppState>>) -> Result<()> {
        match event {
            Event::Key(key) => Self::handle_key_event(key, state),
            Event::Mouse(mouse) => Self::handle_mouse_event(mouse, state),
            _ => Ok(()),
        }
    }

    fn handle_key_event(event: KeyEvent, state: &Arc<Mutex<AppState>>) -> Result<()> {
        let _ = event;
        let _ = state;
        Ok(())
    }

    fn handle_mouse_event(event: MouseEvent, state: &Arc<Mutex<AppState>>) -> Result<()> {
        let _ = event;
        let _ = state;
        Ok(())
    }
}

pub use join::JoinScreen;
pub use settings::SettingsScreen;
pub use splash::SplashScreen;
pub use tutorial::TutorialScreen;
pub use wallets::WalletsScreen;
