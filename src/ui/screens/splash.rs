use std::sync::{Arc, Mutex};

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    message::Message,
    state::{AppState, Screen},
    ui::UIScreen,
};

pub struct SplashScreen {}

impl UIScreen for SplashScreen {
    fn render(frame: &mut ratatui::Frame, state: &Arc<Mutex<AppState>>) {
        let mut state = state.lock().unwrap();
        state.increment();

        let lines = vec![
            Line::from("           ⠰⣉⠆           ").style(Color::Rgb(86, 102, 130)),
            Line::from("      ⢎⡱  ⣀⠤⠤⠤⣀  ⢎⡱      ").style(Color::Rgb(86, 102, 130)),
            Line::from(vec![
                Span::from("▀█▀ █ █ █"),
                Span::from("⠊").style(Color::Rgb(86, 102, 130)),
                Span::from("▟▀█▀▙"),
                Span::from("⠑").style(Color::Rgb(86, 102, 130)),
                Span::from("█ █▀█ ▀█▀"),
            ]),
            Line::from(vec![
                Span::from(" █  █▄█ █"),
                Span::from("⡀").style(Color::Rgb(86, 102, 130)),
                Span::from("▜ ▀ ▛"),
                Span::from("⢀").style(Color::Rgb(86, 102, 130)),
                Span::from("█ █ █  █ "),
            ]),
            Line::from("     ⠰⣉⠆ ⠈⠒⠤⠤⠤⠒⠁ ⠰⣉⠆     ").style(Color::Rgb(86, 102, 130)),
            Line::from("         ⡔⢢   ⡔⢢         ").style(Color::Rgb(86, 102, 130)),
            Line::from("         ⠈⠁   ⠈⠁         ").style(Color::Rgb(86, 102, 130)),
            Line::from(""),
            Line::from(vec![
                Span::from("- ["),
                Span::from("w").style(Style::default().fg(Color::Yellow)),
                Span::from("] Wallets    "),
            ]),
            Line::from(vec![
                Span::from("- ["),
                Span::from("j").style(Style::default().fg(Color::Yellow)),
                Span::from("] Join a Mint"),
            ]),
            Line::from(vec![
                Span::from("- ["),
                Span::from("w").style(Style::default().fg(Color::Yellow)),
                Span::from("] Settings   "),
            ]),
        ];

        let text = Text::from(lines).style(Style::default().fg(Color::Blue));
        let paragraph = Paragraph::new(text).alignment(Alignment::Center);

        let block = Block::bordered()
            .border_style(Style::new().blue())
            .padding(Padding::vertical(1));
        frame.render_widget(
            paragraph.clone().block(block),
            frame
                .area()
                .centered(Constraint::Max(60), Constraint::Max(16)),
        );
    }

    fn handle_keys(
        ev: KeyEvent,
        state: &Arc<Mutex<AppState>>,
        _tx: UnboundedSender<Message>,
    ) -> Result<()> {
        let mut state = AppState::lock(state)?;

        if ev.code == KeyCode::Char('s') {
            state.navigate(Screen::Settings)
        } else if ev.code == KeyCode::Char('w') {
            state.navigate(Screen::Wallets)
        } else if ev.code == KeyCode::Char('j') {
            state.navigate(Screen::Join)
        }

        Ok(())
    }
}
