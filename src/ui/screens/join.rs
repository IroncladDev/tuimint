use std::sync::{Arc, Mutex};

use ratatui::{style::Style, widgets::Block};
use tokio::sync::mpsc::UnboundedSender;

use crate::{message::Message, state::AppState, ui::UIScreen};

pub struct JoinScreen {}

impl UIScreen for JoinScreen {
    fn render(frame: &mut ratatui::Frame, _state: &Arc<Mutex<AppState>>, _tx: UnboundedSender<Message>) {
        let paragraph = ratatui::widgets::Paragraph::new("Join");
        let block = Block::bordered().border_style(Style::new().red().on_white().bold().italic());
        frame.render_widget(paragraph.clone().block(block), frame.area());
    }
}
