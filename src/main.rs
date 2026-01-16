mod backend;
mod message;
mod state;
mod ui;

use backend::handle_messages;
use color_eyre::Result;
use message::Message;
use state::AppState;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

use crate::ui::{handle_events, render};

pub const FRAME_RATE: u64 = 10;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let state = Arc::new(Mutex::new(AppState::new()));
    let (tx, rx) = mpsc::unbounded_channel::<Message>();

    let backend_state = state.clone();

    // Handles messages passed from the UI to the backend
    tokio::spawn(handle_messages(rx, backend_state));
    ratatui::run(|terminal| -> Result<()> {
        loop {
            terminal.draw(|frame| render(frame, &state))?;
            handle_events(&state, tx.clone())?;
            if state.lock().unwrap().should_quit {
                break;
            }
        }
        Ok(())
    })
    .unwrap_or_else(|e| panic!("TUIMint exited with error: {}", e));

    Ok(())
}
