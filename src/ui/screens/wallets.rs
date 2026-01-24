use std::sync::{Arc, Mutex};

use ratatui::{style::Style, widgets::Block};
use tokio::sync::mpsc::UnboundedSender;

use crate::{message::Message, state::AppState, ui::screens::UIScreen};

pub struct WalletsScreen {}

impl UIScreen for WalletsScreen {
    fn render(
        frame: &mut ratatui::Frame,
        _state: &Arc<Mutex<AppState>>,
        _tx: UnboundedSender<Message>,
    ) {
        let paragraph = ratatui::widgets::Paragraph::new("Wallets");
        let block = Block::bordered().border_style(Style::new().cyan().on_white().bold().italic());
        frame.render_widget(paragraph.clone().block(block), frame.area());
    }
}
