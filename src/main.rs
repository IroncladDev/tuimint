mod backend;
mod message;
mod state;
mod ui;

use anyhow::Result;
use backend::handle_messages;
use crossterm::{
    event::{
        DisableFocusChange, DisableMouseCapture, EnableFocusChange, EnableMouseCapture, Event,
        KeyCode, KeyModifiers, poll, read,
    },
    execute,
};
use message::Message;
use ratatui::Frame;
use state::AppState;
use state::Screen;
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::{Duration, Instant},
};
use tokio::sync::mpsc::{self, UnboundedSender};
use ui::screens::*;

pub const FRAME_RATE: u64 = 60;

#[tokio::main]
async fn main() {
    execute!(std::io::stdout(), EnableMouseCapture, EnableFocusChange).ok();

    let state = Arc::new(Mutex::new(AppState::new()));
    let backend_state = state.clone();
    let (tx, rx) = mpsc::unbounded_channel::<Message>();

    // Handles messages passed from the UI to the backend
    tokio::spawn(handle_messages(rx, backend_state));

    ratatui::run(|terminal| {
        let framerate = Duration::from_millis(1000 / FRAME_RATE);

        loop {
            let start = Instant::now();

            terminal
                .draw(|frame| render_screen(frame, &state, tx.clone()))
                .ok();

            if let Ok(true) = poll(Duration::ZERO)
                && let Ok(event) = read()
            {
                if let Event::Key(key) = event
                    && key.code == KeyCode::Char('c')
                    && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    break;
                }

                handle_events(&state, event).ok();
            }

            let elapsed = start.elapsed();
            if elapsed < framerate {
                sleep(framerate - elapsed);
            }
        }
    });

    execute!(std::io::stdout(), DisableMouseCapture, DisableFocusChange).ok();
}

fn handle_events(state: &Arc<Mutex<AppState>>, event: Event) -> Result<()> {
    let screen = state.lock().unwrap().screen;

    match screen {
        Screen::Splash => SplashScreen::handle_event(event, state),
        Screen::Join => JoinScreen::handle_event(event, state),
        Screen::Wallets => WalletsScreen::handle_event(event, state),
        Screen::Settings => SettingsScreen::handle_event(event, state),
        Screen::Tutorial => TutorialScreen::handle_event(event, state),
    }
}

fn render_screen(frame: &mut Frame, state: &Arc<Mutex<AppState>>, tx: UnboundedSender<Message>) {
    let screen = state.lock().unwrap().screen;
    match screen {
        Screen::Splash => SplashScreen::render(frame, state, tx),
        Screen::Join => JoinScreen::render(frame, state, tx),
        Screen::Wallets => WalletsScreen::render(frame, state, tx),
        Screen::Settings => SettingsScreen::render(frame, state, tx),
        Screen::Tutorial => TutorialScreen::render(frame, state, tx),
    }
}
