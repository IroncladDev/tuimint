pub mod join;
pub mod settings;
pub mod splash;
pub mod tutorial;
pub mod wallets;

use std::sync::{Arc, Mutex};

use crate::{message::Message, state::AppState};
use tokio::sync::mpsc::UnboundedSender;

pub trait UIScreen {
    fn render(
        frame: &mut ratatui::Frame,
        state: &Arc<Mutex<AppState>>,
        tx: UnboundedSender<Message>,
    );
}

pub use join::JoinScreen;
pub use settings::SettingsScreen;
pub use splash::SplashScreen;
pub use tutorial::TutorialScreen;
pub use wallets::WalletsScreen;
