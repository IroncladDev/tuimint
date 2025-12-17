

use crate::types::Wallets;

mod database;
mod types;

#[tokio::main]
pub async fn main() {
    let mut wallets = Wallets::new().await.expect("Should unwrap");
    //
    // for item in wallets.show_mnemonic().await.expect("lol") {
    //     println!("{:?}", item)
    // }

    let _ = wallets.load_configs().await;

    // let _ = wallets.join("fed11qgqzggnhwden5te0v9cxjtn9vd3jue3wvfkxjmnyva6kzunyd9skutnwv46z7qqpyzhv5mxgpl79xz7j649sj6qldmde5s2uxchy4uh7840qgymsqmazzp6sn43").await;

    if let Ok(clients) = wallets.get_clients()
        && clients.is_empty()
    {
        println!("EMPTY")
    } else {
        println!("NOT EMPTY")
    }

    // TODO: populate `clients`
    // wallets.clients.lock().await.get("")
}
// fed11qgqzggnhwden5te0v9cxjtn9vd3jue3wvfkxjmnyva6kzunyd9skutnwv46z7qqpyzhv5mxgpl79xz7j649sj6qldmde5s2uxchy4uh7840qgymsqmazzp6sn43
// aeca6cc80ffc530bd2d54b09681f6edb9a415c362e4af2fe3d5e04137006fa21

// wallets.join("fed11qgqzggnhwden5te0v9cxjtn9vd3jue3wvfkxjmnyva6kzunyd9skutnwv46z7qqpyzhv5mxgpl79xz7j649sj6qldmde5s2uxchy4uh7840qgymsqmazzp6sn43").await.expect("whatever comes");
//

// let invite = "fed11qgqrgvnhwden5te0v9k8q6rp9ekh2arfdeukuet595cr2ttpd3jhq6rzve6zuer9wchxvetyd938gcewvdhk6tcqqysptkuvknc7erjgf4em3zfh90kffqf9srujn6q53d6r056e4apze5cw27h75";
// let id = "15db8cb4f1ec8e484d73b889372bec94812580f929e8148b7437d359af422cd3";
//
// let open_result = wallets
//     .open(FederationIdKey {
//         id: FederationId::from_str(
//             id
//         ).expect("should unwrap"),
//     })
//     .await;
//
// if open_result.is_ok() {
//     println!("{:?}", open_result);
// } else {
//     wallets.join(invite).await.expect("whatever comes");
