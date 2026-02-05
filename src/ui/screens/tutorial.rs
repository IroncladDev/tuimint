use crate::state::Screen;
use crate::ui::prelude::*;
use ratatui::prelude::*;
use ratatui::widgets::*;

pub struct TutorialScreen {}

impl Component for TutorialScreen {
    fn render(&mut self, frame: &mut Frame, state: &AppStateMutex) {
        let state = state.lock().unwrap();

        if state.screen != Screen::Tutorial {
            return;
        }

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
