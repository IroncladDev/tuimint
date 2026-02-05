mod component;
mod root;
mod screens;

pub use component::Component;
pub use root::Root;

pub mod prelude {
    pub use super::Component;
    pub use crate::message::Message;
    pub use crate::types::*;
    pub use crossterm::event::{KeyEvent, MouseEvent, KeyCode};
}
