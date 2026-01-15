mod backend;
mod message;
mod state;
mod ui;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use backend::handle_messages;
use color_eyre::Result;
use color_eyre::eyre::Context;
use crossterm::event::{self};
use message::Message;
use ratatui::DefaultTerminal;
use tokio::sync::mpsc::{self, UnboundedSender};
use ui::{handle_event, render};

use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let state = Arc::new(Mutex::new(AppState::new()));
    let (tx, rx) = mpsc::unbounded_channel::<Message>();

    let backend_state = state.clone();

    tokio::spawn(handle_messages(rx, backend_state));
    ratatui::run(|term| run(term, &state, tx))
        .unwrap_or_else(|e| panic!("AAAAA exited with error: {}", e));

    Ok(())
}

fn run(
    terminal: &mut DefaultTerminal,
    state: &Arc<Mutex<AppState>>,
    tx: UnboundedSender<Message>,
) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, state))?;
        if event::poll(Duration::from_millis(16))? {
            let ev = event::read()?;
            handle_event(ev, state, tx.clone())?;
        }
        if state.lock().unwrap().should_quit {
            break;
        }
    }
    Ok(())
}
