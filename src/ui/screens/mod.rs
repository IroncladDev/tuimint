pub mod join;
pub mod settings;
pub mod splash;
pub mod wallets;

use std::sync::{Arc, Mutex};

use crate::{message::Message, state::AppState};

pub trait UIScreen {
    fn render(
        frame: &mut ratatui::Frame,
        state: &Arc<Mutex<AppState>>,
        tx: UnboundedSender<Message>,
    );
}

pub use join::*;
pub use settings::*;
pub use splash::*;
use tokio::sync::mpsc::UnboundedSender;
pub use wallets::*;
