use std::sync::{Arc, Mutex};

use ratatui::{
    layout::{Alignment, Constraint},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{message::Message, state::AppState, ui::screens::UIScreen};

pub struct SplashScreen {}

impl UIScreen for SplashScreen {
    fn render(
        frame: &mut ratatui::Frame,
        _state: &Arc<Mutex<AppState>>,
        _tx: UnboundedSender<Message>,
    ) {
        let mut lines = vec![
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
        ];

        lines.push(Line::from(vec![
            Span::from("- ["),
            Span::from("t").style(Style::default().fg(Color::Yellow)),
            Span::from("] What is Fedimint?"),
        ]));

        lines.push(Line::from(vec![
            Span::from("- ["),
            Span::from("j").style(Style::default().fg(Color::Yellow)),
            Span::from("] Join a Mint"),
        ]));

        // TODO: show wallets if any
        if false {
            lines.push(Line::from(vec![
                Span::from("- ["),
                Span::from("w").style(Style::default().fg(Color::Yellow)),
                Span::from("] Wallets    "),
            ]));
        }

        lines.push(Line::from(vec![
            Span::from("- ["),
            Span::from("w").style(Style::default().fg(Color::Yellow)),
            Span::from("] Settings   "),
        ]));

        let text = Text::from(lines).style(Style::default().fg(Color::Blue));
        let paragraph = Paragraph::new(text).alignment(Alignment::Center);

        let block = Block::bordered()
            .border_style(Style::new().blue())
            .padding(Padding::vertical(1));
        frame.render_widget(
            paragraph.clone().block(block),
            frame
                .area()
                .centered(Constraint::Max(60), Constraint::Max(18)),
        );
    }
}
