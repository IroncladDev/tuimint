mod screens;

use crate::{message::Message, state::AppState};
use anyhow::Result;
use crossterm::event::{Event, KeyEvent, MouseEvent};
use screens::JoinScreen;
use screens::SettingsScreen;
use screens::SplashScreen;
use screens::TutorialScreen;
use screens::WalletsScreen;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedSender;

pub struct Root {
    children: Vec<Box<dyn Component>>,
}

impl Root {
    pub fn new() -> Self {
        Self {
            children: vec![
                Box::new(SplashScreen::new()),
                Box::new(TutorialScreen {}),
                Box::new(JoinScreen {}),
                Box::new(WalletsScreen {}),
                Box::new(SettingsScreen {}),
            ]
        }
    }
}

impl Component for Root {
    fn children(&mut self) -> &mut [Box<dyn Component>] {
        &mut self.children
    }
}

pub trait Component {
    fn children(&mut self) -> &mut [Box<dyn Component>] {
        &mut []
    }

    fn draw(&self, frame: &mut ratatui::Frame, state: &Arc<Mutex<AppState>>) {
        let _ = frame;
        let _ = state;
    }

    fn render(&mut self, frame: &mut ratatui::Frame, state: &Arc<Mutex<AppState>>) {
        self.draw(frame, state);

        for child in self.children() {
            child.render(frame, state);
        }
    }

    fn handle_event(
        &mut self,
        event: Event,
        state: &Arc<Mutex<AppState>>,
        tx: UnboundedSender<Message>,
    ) -> Result<()> {
        for child in self.children() {
            child.handle_event(event.clone(), state, tx.clone())?;
        }

        match event {
            Event::Key(key) => self.handle_key_event(key, state, tx),
            Event::Mouse(mouse) => self.handle_mouse_event(mouse, state, tx),
            _ => Ok(()),
        }
    }

    fn handle_key_event(
        &mut self,
        event: KeyEvent,
        state: &Arc<Mutex<AppState>>,
        tx: UnboundedSender<Message>,
    ) -> Result<()> {
        let _ = event;
        let _ = state;
        let _ = tx;
        Ok(())
    }

    fn handle_mouse_event(
        &mut self,
        event: MouseEvent,
        state: &Arc<Mutex<AppState>>,
        tx: UnboundedSender<Message>,
    ) -> Result<()> {
        let _ = event;
        let _ = state;
        let _ = tx;
        Ok(())
    }
}

