mod refresh_clients;

use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::UnboundedReceiver;
use crate::{message::Message, state::AppState};

/// Handles messages sent from the UI
/// Used for triggering asynchronous backend actions
pub async fn handle_messages(mut rx: UnboundedReceiver<Message>, state: Arc<Mutex<AppState>>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            Message::RefreshClients => refresh_clients::refresh_clients(&state).await,
            // Message::RefreshWallets(client_id) => {
            //     state.wallets.clear();
            //     state.wallets.push(client_id);
            // },
            _ => {}
        }
    }
}
