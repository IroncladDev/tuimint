mod backend;
mod message;
mod state;
mod ui;
mod types;

use crate::ui::{Component, Root};
use crossterm::event::{Event, KeyCode, KeyModifiers, poll, read};
use message::Message;
use state::AppState;
use std::{
    panic,
    sync::{Arc, Mutex},
    thread::sleep,
    time::{Duration, Instant},
};
use tokio::sync::mpsc;
use backend::handle_messages;

pub const FRAME_RATE: u64 = 30;

#[tokio::main]
async fn main() {
    // Enable mouse capture
    // execute!(std::io::stdout(), EnableMouseCapture).ok();

    // Install a panic hook to reset the terminal in case of a panic
    panic::set_hook(Box::new(|info| {
        // reset_terminal();

        eprintln!("TUIMint panicked with error: {}", info)
    }));

    let state = Arc::new(Mutex::new(AppState::new()));
    let backend_state = state.clone();
    let (tx, rx) = mpsc::unbounded_channel::<Message>();

    // Handles messages passed from the UI to the backend
    tokio::spawn(handle_messages(rx, backend_state));

    // Main UI Component
    let mut root = Root::new(&state, tx.clone());

    ratatui::run(|terminal| {
        let framerate = Duration::from_millis(1000 / FRAME_RATE);

        loop {
            let start = Instant::now();

            terminal.draw(|frame| root.render(frame, &state)).ok();

            if let Ok(true) = poll(Duration::ZERO)
                && let Ok(event) = read()
            {
                if let Event::Key(key) = event
                    && key.code == KeyCode::Char('c')
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    break;
                }

                root.handle_event(event, &state, tx.clone()).ok();
            }

            let elapsed = start.elapsed();
            if elapsed < framerate {
                sleep(framerate - elapsed);
            }
        }
    });

    // reset_terminal();
}

// fn reset_terminal() {
//     execute!(std::io::stdout(), DisableMouseCapture).ok();
// }
