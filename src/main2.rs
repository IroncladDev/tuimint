mod wallet;

use std::str::FromStr;

use fedimint_core::Amount;
use fedimint_mint_client::OOBNotes;
use wallet::Wallet;

use crate::wallet::WalletError;

#[tokio::main]
pub async fn main() -> Result<(), WalletError> {
    let mut wallet = Wallet::new();
    let wallet = wallet.open().await?;
    // let wallet = wallet
    // .join("fed11qgqrgvnhwden5te0v9k8q6rp9ekh2arfdeukuet595cr2ttpd3jhq6rzve6zuer9wchxvetyd938gcewvdhk6tcqqysptkuvknc7erjgf4em3zfh90kffqf9srujn6q53d6r056e4apze5cw27h75").await?;

    let balance = wallet.balance().await?;
    println!("{:?}", balance);

    // let operation = wallet.receive_ecash("AwEEFduMtAD9AlMHQAGI0TlG7MQb-fL84PJ5pyRPsRZDv6v__s5a0IcImH6yE7FdYPNE5nrIluzkK_7rxX5nmECIgffBSb3kM5qUyddkApNFEcjseKRd78UzWusRU_0CAAGpIMxELfCR63tBHjLkuAicaFCIrtGk6Cwwf8uiVvd6a9-WfM80ecV7B6tdurKOJ5tUG47369-ytwqZOWLUaw8bzZQydbOorPTdWHlUMtiBCv1AAAGOF41YZtK8ptU12o5Opp3eJs0EEVtTJRsc8OTLcQmzFgDEOiwIAje1OGZ7o004MRHcKIojWMsPozAnVJA_PhcQg5oHe_ThVtHjeUSR0A6iXv4AAQAAAZCihtJGLTgo_jQEiHb9N1GAsv9-spgkGWhebE1k_B9xBqTdcNQ8SsbIwg9QBIO7L52vosZttd5j0KZ2zywpTSZw5T0RInsAE3cIijRBPMYb_gACAAABqx-4haolmdB8IDNny3XDXniQXqr4WPcqXz0EKtyosIfw6SUC4F0XU-Si9jXPlivvVYBB1PbcST8Ym_sjsrF9XTgoTNYnWZq0D0-WQ_N5P2P-AAQAAAGt0Y3Www5FEI7F5pkKenwIA4KkcpczaR9Kyljg6JaGd0CH6D8gEEqUzXPwQ4CFb9cGy9rWWGMO6kTjZHJNFgY-2awb-K5T59u5j4gxWx-DHf4ACAAAAa3E5yM2pBfHSujxcUfkIjEfVH8PpPZ4dpMQVSI8089sQF6dodrDqE3J_zzlW4EShKDQ6asINqoNgYCwZJbG0K2ois-NTrQUAoJET_ZOdjfBAlUBADJ3c3M6Ly9hbHBoYS5tdXRpbnluZXQtMDUtYWxlcGhiZnQuZGV2LmZlZGlidGMuY29tLxXbjLTx7I5ITXO4iTcr7JSBJYD5KegUi3Q301mvQizT").await;
    // println!("{:?}", operation);
    
    let (operation, notes) = wallet.spend_ecash(Amount::from_sats(10)).await?;
    println!("{:?}", operation);
    println!("{:?}", notes);

    let new_balance = wallet.balance().await?;
    println!("{:?}", new_balance);

    Ok(())
}

// fed11qgqrgvnhwden5te0v9k8q6rp9ekh2arfdeukuet595cr2ttpd3jhq6rzve6zuer9wchxvetyd938gcewvdhk6tcqqysptkuvknc7erjgf4em3zfh90kffqf9srujn6q53d6r056e4apze5cw27h75
