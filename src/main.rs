mod database;
mod state;
mod types;

use std::sync::{Arc, Mutex};
use std::time::Duration;

use color_eyre::Result;
use color_eyre::eyre::Context;
use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::Paragraph;
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::state::AppState;

enum Message {
    Test,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let state = Arc::new(Mutex::new(AppState::new()));
    let (tx, rx) = mpsc::unbounded_channel();

    let backend_state = state.clone();

    tokio::spawn(receive_messages(rx, backend_state));
    ratatui::run(|term| run(term, &state, tx)).context("Failed to run app")?;

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

fn render(frame: &mut Frame, state: &Arc<Mutex<AppState>>) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    let count = Paragraph::new(format!("Count: {}", state.lock().unwrap().count));
    frame.render_widget(greeting, frame.area());
    frame.render_widget(count, frame.area());
}

fn handle_event(
    ev: Event,
    state: &Arc<Mutex<AppState>>,
    tx: UnboundedSender<Message>,
) -> Result<()> {
    if let Event::Key(key) = ev {
        match key.code {
            KeyCode::Char('q') => state.lock().unwrap().quit(),
            KeyCode::Char(' ') => tx.send(Message::Test).unwrap(),
            _ => {}
        }
    }
    Ok(())
}

async fn receive_messages(mut rx: UnboundedReceiver<Message>, state: Arc<Mutex<AppState>>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            Message::Test => state.lock().unwrap().increment(),
        }
    }
}
