use crate::types::*;
use anyhow::Result;
use crossterm::event::{Event, KeyEvent, MouseEvent};
use ratatui::Frame;

pub trait Component {
    /// A list of child components
    fn children(&mut self) -> &mut [Box<dyn Component>] {
        &mut []
    }

    /// Draws the component
    fn draw(&self, frame: &mut Frame, state: &AppStateMutex) {
        let _ = frame;
        let _ = state;
    }

    /// Handles a key event
    fn on_key_event(&mut self, event: KeyEvent, state: &AppStateMutex, tx: TxSender) -> Result<()> {
        let _ = event;
        let _ = state;
        let _ = tx;
        Ok(())
    }

    /// Handles a mouse event
    fn on_mouse_event(
        &mut self,
        event: MouseEvent,
        state: &AppStateMutex,
        tx: TxSender,
    ) -> Result<()> {
        let _ = event;
        let _ = state;
        let _ = tx;
        Ok(())
    }

    /// Draws the Component and all its children
    fn render(&mut self, frame: &mut Frame, state: &AppStateMutex) {
        self.draw(frame, state);

        for child in self.children() {
            child.render(frame, state);
        }
    }

    /// Handles terminal events
    fn handle_event(&mut self, event: Event, state: &AppStateMutex, tx: TxSender) -> Result<()> {
        for child in self.children() {
            child.handle_event(event.clone(), state, tx.clone())?;
        }

        match event {
            Event::Key(key) => self.on_key_event(key, state, tx),
            Event::Mouse(mouse) => self.on_mouse_event(mouse, state, tx),
            _ => Ok(()),
        }
    }
}
