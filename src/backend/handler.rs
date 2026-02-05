use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::UnboundedReceiver;

use crate::{message::Message, state::AppState};

/// Handles messages sent from the UI
/// Used for triggering asynchronous backend actions
pub async fn handle_messages(mut rx: UnboundedReceiver<Message>, state: Arc<Mutex<AppState>>) {
    while let Some(msg) = rx.recv().await {
        let mut state = state.lock().unwrap();
        match msg {
            Message::Increment => state.count += 1,
            Message::Decrement => state.count -= 1,
            Message::Double => state.count *= 2,
        }
    }
}
