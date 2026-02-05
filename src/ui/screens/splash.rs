use crate::message::Message;
use crate::state::AppState;
use crate::state::Screen;
use crate::ui::Component;
use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::*;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedSender;

pub struct SplashScreen {}

impl SplashScreen {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component for SplashScreen {
    fn draw(&self, frame: &mut ratatui::Frame, state: &Arc<Mutex<AppState>>) {
        let state = state.lock().unwrap();

        if state.screen != Screen::Splash {
            return;
        }

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
            .padding(Padding::vertical(1))
            .title_bottom(" CTRL+C to exit ")
            .title_alignment(Alignment::Center);
        frame.render_widget(
            paragraph.clone().block(block),
            frame
                .area()
                .centered(Constraint::Max(60), Constraint::Max(18)),
        );
    }

    fn handle_key_event(
        &mut self,
        event: crossterm::event::KeyEvent,
        state: &Arc<Mutex<AppState>>,
        _tx: UnboundedSender<Message>,
    ) -> anyhow::Result<()> {
        let mut state = state.lock().unwrap();

        if state.screen != Screen::Splash {
            return Ok(());
        }

        if let KeyCode::Char('j') = event.code {
            state.navigate(Screen::Join);
        }

        if let KeyCode::Char('w') = event.code {
            state.navigate(Screen::Wallets);
        }

        if let KeyCode::Char('t') = event.code {
            state.navigate(Screen::Tutorial);
        }

        Ok(())
    }
}
