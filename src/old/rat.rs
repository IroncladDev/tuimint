use std::time::Duration;

use color_eyre::{Result, eyre::Context};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode},
    widgets::Paragraph,
};

mod types;
mod db;

use types::{Wallet, Wallets};


/// This is a bare minimum example. There are many approaches to running an application loop, so
/// this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
/// teardown of a terminal application.
///
/// This example does not handle events or update the application state. It just draws a greeting
/// and exits when the user presses 'q'.
fn main() -> Result<()> {
    color_eyre::install()?; // augment errors / panics with easy to read messages
    let terminal = ratatui::init();
    let mut state = State::new();
    let app_result = run(terminal, &mut state).context("app loop failed");
    ratatui::restore();
    app_result
}

/// Run the application loop. This is where you would handle events and update the application
/// state. This example exits when the user presses 'q'. Other styles of application loops are
/// possible, for example, you could have multiple application states and switch between them based
/// on events, or you could have a single application state and update it based on events.
fn run(mut terminal: DefaultTerminal, state: &mut State) -> Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, state))?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

struct State {
    pub count: i32,
}

impl State {
    pub fn new() -> Self {
        State { count: 0 }
    }

    pub fn increase(&mut self) {
        self.count += 1;
    }
}

/// Render the application. This is where you would draw the application UI. This example draws a
/// greeting.
fn draw(frame: &mut Frame, state: &mut State) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    let greeting2 = Paragraph::new(format!("Count: {}", state.count));
    frame.render_widget(greeting, frame.area());
    frame.render_widget(greeting2, frame.area());
    state.increase();
}

/// Check if the user has pressed 'q'. This is where you would handle events. This example just
/// checks if the user has pressed 'q' and returns true if they have. It does not handle any other
/// events. There is a 250ms timeout on the event poll to ensure that the terminal is rendered at
/// least once every 250ms. This allows you to do other work in the application loop, such as
/// updating the application state, without blocking the event loop for too long.
fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(33)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
