use std::str::FromStr;

use fedimint_bip39::{Bip39RootSecretStrategy, Mnemonic};
use fedimint_client::{Client, RootSecret, secret::RootSecretStrategy};
use fedimint_core::{
    db::{Database, IDatabaseTransactionOpsCoreTyped},
    invite_code::InviteCode,
};
use fedimint_cursed_redb::MemAndRedb;

use crate::{
    database::{FederationConfig, FederationIdKey},
    types::{Error, Result, Wallet},
};
use rand::thread_rng;

// TODO: look into anyhow

#[derive(Debug)]
pub struct Wallets {
    wallets: Vec<Wallet>,
    active_wallet_id: Option<String>,
    db: Database,
}

impl Wallets {
    // Should
    // load all ids from db
    // load all wallets from ids
    // set active wallet to first wallet if any
    pub async fn new() -> Result<Wallets> {
        if let Some(db_dir) = dirs::data_local_dir() {
            let db_path = db_dir.join("tuimint/");
            let db_file = db_path.join("tuimint.db");

            let _ = tokio::fs::create_dir_all(&db_path).await;
            let cursed_db = MemAndRedb::new(db_file)
                .await
                .map_err(|_| Error::DBLoadError)?;
            let db = Database::new(cursed_db, Default::default());

            Ok(Wallets {
                wallets: Vec::new(),
                active_wallet_id: None,
                db,
            })
        } else {
            Err(Error::DBLoadError)
        }
    }

    pub async fn get_active_wallet(&mut self) -> Option<&Wallet> {
        self.wallets.iter().find(|wallet| {
            if let Some(id) = &self.active_wallet_id {
                return wallet.federation_id.id.to_string() == *id;
            }
            false
        })
    }

    // pub async fn switch(&mut self, federation: FederationIdKey) {}

    pub async fn join(&mut self, invite_code: &str) -> Result<()> {
        let secret = self.mnemonic_secret().await?;
        let wallet = Wallet::from_joined(invite_code, secret).await?;
        let invite_code =
            InviteCode::from_str(invite_code).map_err(|_| Error::InvalidInviteCode)?;
        let id_str = wallet.federation_id.id.to_string();

        let config = FederationConfig { invite_code };
        let id = config.invite_code.federation_id();
        let mut dbtx = self.db.begin_transaction().await;
        dbtx.insert_entry(&FederationIdKey { id }, &config).await;
        dbtx.commit_tx_result()
            .await
            .map_err(|_| Error::DBLoadError)?;

        self.wallets.push(wallet);
        self.active_wallet_id = Some(id_str.clone());

        println!("{}", id.clone());

        Ok(())
    }

    pub async fn open(&mut self, federation_id: FederationIdKey) -> Result<()> {
        let secret = self.mnemonic_secret().await?;
        let wallet = Wallet::from_opened(federation_id, secret).await?;
        let id = wallet.federation_id.id.to_string();

        self.wallets.push(wallet);
        self.active_wallet_id = Some(id.clone());

        println!("{}", id.clone());

        Ok(())
    }

    // async fn save(mut dbtx: DatabaseTransaction<'_, Committable>) {}

    async fn mnemonic_secret(&self) -> Result<RootSecret> {
        let mnemonic = self.load_or_generate_mnemonic().await?;

        Ok(RootSecret::StandardDoubleDerive(Bip39RootSecretStrategy::<
            12,
        >::to_root_secret(
            &mnemonic
        )))
    }

    pub async fn show_mnemonic(&self) -> Result<Vec<String>> {
        let mnemonic = self.load_or_generate_mnemonic().await?;
        let mut words: Vec<String> = Vec::new();

        for word in mnemonic.words() {
            words.push(word.to_string());
        }

        Ok(words)
    }

    pub async fn load_or_generate_mnemonic(&self) -> Result<Mnemonic> {
        Ok(
            if let Ok(entropy) = Client::load_decodable_client_secret::<Vec<u8>>(&self.db).await {
                Mnemonic::from_entropy(&entropy).map_err(|_| Error::MnemonicError)?
            } else {
                let mnemonic = Bip39RootSecretStrategy::<12>::random(&mut thread_rng());
                Client::store_encodable_client_secret(&self.db, mnemonic.to_entropy())
                    .await
                    .map_err(|_| Error::MnemonicError)?;
                mnemonic
            },
        )
    }

    // pub async fn leave(&mut self, federation: FederationIdKey) {}
    // pub async fn recover(&mut self, federation: FederationIdKey) {}
}
