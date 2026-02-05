use std::sync::{Arc, Mutex};
use crate::state::AppState;

pub async fn refresh_clients(state: &Arc<Mutex<AppState>>) {
    println!("refresh clients");
}
