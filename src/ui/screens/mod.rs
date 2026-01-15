pub mod splash;
pub mod join;
pub mod wallets;
pub mod settings;

use std::sync::{Arc, Mutex};

use crossterm::event::{KeyEvent};
use tokio::sync::mpsc::UnboundedSender;

use crate::{message::Message, state::AppState};
use color_eyre::Result;

pub trait UIScreen {
    fn render(frame: &mut ratatui::Frame, state: &Arc<Mutex<AppState>>);
    fn handle_keys(ev: KeyEvent, state: &Arc<Mutex<AppState>>, tx: UnboundedSender<Message>) -> Result<()>;
}

pub use splash::*;
pub use join::*;
pub use wallets::*;
pub use settings::*;
