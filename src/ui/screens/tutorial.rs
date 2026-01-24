use std::sync::{Arc, Mutex};

use ratatui::{
    layout::{Alignment, Constraint},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

use crate::{message::Message, state::AppState, ui::screens::UIScreen};

pub struct TutorialScreen {}

impl UIScreen for TutorialScreen {
    fn render(
        frame: &mut ratatui::Frame,
        state: &Arc<Mutex<AppState>>,
        _tx: UnboundedSender<Message>,
    ) {
        let mut state = state.lock().unwrap();

        let lines = vec![Line::from("Tutorial")];

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
