mod backend;
mod message;
mod state;
mod ui;

use backend::handle_messages;
use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, poll};
use message::Message;
use state::AppState;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::sync::mpsc;

use crate::ui::render;

pub const FRAME_RATE: u64 = 60;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let state = Arc::new(Mutex::new(AppState::new()));
    let (tx, rx) = mpsc::unbounded_channel::<Message>();
    let mut last = Instant::now();

    let backend_state = state.clone();

    // Handles messages passed from the UI to the backend
    tokio::spawn(handle_messages(rx, backend_state));
    ratatui::run(|terminal| -> Result<()> {
        'outer: loop {
            let now = Instant::now();
            if now.duration_since(last) >= Duration::from_millis(1000 / FRAME_RATE) {
                last = now;

                terminal.draw(|frame| render(frame, &state, tx.clone()))?;

                {
                    let mut state = AppState::lock(&state)?;
                    state.keys_typed.clear();
                    state.modifiers_held = KeyModifiers::empty();
                }
            }

            while poll(Duration::ZERO)? {
                if let Event::Key(key) = crossterm::event::read()? {
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        break 'outer;
                    }

                    let mut state = AppState::lock(&state)?;

                    state.modifiers_held = key.modifiers;
                    state.keys_typed.insert(key.code);
                }
            }
        }
        Ok(())
    })
    .unwrap_or_else(|e| panic!("TUIMint exited with error: {}", e));

    Ok(())
}
