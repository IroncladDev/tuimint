use std::sync::{Arc, Mutex};

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{message::Message, state::AppState, ui::UIScreen};

pub struct SplashScreen {}

impl UIScreen for SplashScreen {
    fn render(frame: &mut ratatui::Frame, state: &Arc<Mutex<AppState>>, tx: UnboundedSender<Message>) {
        let mut state = state.lock().unwrap();

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
            Line::from(format!(
                "types: {:?}, modifiers: {:?}",
                state.keys_typed, state.modifiers_held
            )),
            Line::from(format!("count: {}", state.count)),
        ];

        if state.keys_typed.contains(&KeyCode::Char('c')) {
            let _ = tx.send(Message::Double);
        }

        let text = Text::from(lines).style(Style::default().fg(Color::Blue));
        let paragraph = Paragraph::new(text).alignment(Alignment::Center);

        let block = Block::bordered()
            .border_style(Style::new().blue())
            .padding(Padding::vertical(1));
        frame.render_widget(
            paragraph.clone().block(block),
            frame
                .area()
                .centered(Constraint::Max(60), Constraint::Max(17)),
        );
    }
}
