pub enum Message {
    /// Refreshes the list of fedimint clients
    RefreshClients,
    /// Refreshes the list of fedimint wallets for a given client
    RefreshWallets(String),
}
