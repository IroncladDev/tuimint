use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::UnboundedSender;

use crate::{message::Message, state::AppState};

pub type AppStateMutex = Arc<Mutex<AppState>>;
pub type TxSender = UnboundedSender<Message>;
