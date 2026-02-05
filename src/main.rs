mod backend;
mod message;
mod state;
mod ui;

use backend::handle_messages;
use crossterm::{
    event::{
        DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture, Event,
        KeyCode, KeyModifiers, poll, read,
    },
    execute,
};
use message::Message;
use state::AppState;
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::{Duration, Instant},
};
use tokio::sync::mpsc;

use crate::ui::{Component, Root};

pub const FRAME_RATE: u64 = 60;

#[tokio::main]
async fn main() {
    execute!(std::io::stdout(), EnableMouseCapture, EnableFocusChange).ok();

    let state = Arc::new(Mutex::new(AppState::new()));
    let backend_state = state.clone();
    let (tx, rx) = mpsc::unbounded_channel::<Message>();

    // Handles messages passed from the UI to the backend
    tokio::spawn(handle_messages(rx, backend_state));

    let mut main = Root::new();

    ratatui::run(|terminal| {
        let framerate = Duration::from_millis(1000 / FRAME_RATE);

        loop {
            let start = Instant::now();

            terminal.draw(|frame| main.render(frame, &state)).ok();

            if let Ok(true) = poll(Duration::ZERO)
                && let Ok(event) = read()
            {
                if let Event::Key(key) = event
                    && key.code == KeyCode::Char('c')
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    break;
                }

                main.handle_event(event, &state, tx.clone()).ok();
            }

            let elapsed = start.elapsed();
            if elapsed < framerate {
                sleep(framerate - elapsed);
            }
        }
    });

    execute!(std::io::stdout(), DisableMouseCapture, DisableFocusChange).ok();
}
