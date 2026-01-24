use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::UnboundedReceiver;

use crate::{message::Message, state::AppState};

/// Handles messages sent from the UI
/// Used for triggering asynchronous backend actions
pub async fn handle_messages(mut rx: UnboundedReceiver<Message>, _state: Arc<Mutex<AppState>>) {
    while let Some(_msg) = rx.recv().await {
        todo!("Async operations");
        // let mut state = state.lock().unwrap();
        // match msg {
        //     _ => {}
        // }
    }
}
