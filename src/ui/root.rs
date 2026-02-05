use super::prelude::*;
use crate::ui::screens::*;

pub struct Root {
    children: Vec<Box<dyn Component>>,
}

impl Root {
    pub fn new(_state: &AppStateMutex, tx: TxSender) -> Self {
        tx.send(Message::RefreshClients).ok();

        Self {
            children: vec![
                Box::new(SplashScreen::new()),
                Box::new(TutorialScreen {}),
                Box::new(JoinScreen {}),
                Box::new(WalletsScreen {}),
                Box::new(SettingsScreen {}),
            ],
        }
    }
}

impl Component for Root {
    fn children(&mut self) -> &mut [Box<dyn Component>] {
        &mut self.children
    }
}
