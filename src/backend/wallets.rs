use anyhow::{Result, anyhow};
use std::{
    collections::BTreeMap,
    str::FromStr,
    sync::{Arc, Mutex, MutexGuard},
};

use fedimint_bip39::{Bip39RootSecretStrategy, Mnemonic};
use fedimint_client::{Client, ClientHandleArc, RootSecret, secret::RootSecretStrategy};
use fedimint_core::{
    config::FederationId,
    db::{Database, IDatabaseTransactionOpsCoreTyped},
    invite_code::InviteCode,
};
use fedimint_cursed_redb::MemAndRedb;
use futures::StreamExt;

use crate::backend::{FederationConfig, FederationIdKey, FederationIdKeyPrefix, Wallet};
use rand::thread_rng;

// TODO: look into anyhow

#[derive(Debug)]
pub struct Wallets {
    pub clients: Arc<Mutex<BTreeMap<FederationId, ClientHandleArc>>>,
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
            let cursed_db = MemAndRedb::new(db_file).await?;
            let db = Database::new(cursed_db, Default::default());

            Ok(Wallets {
                clients: Arc::new(Mutex::new(BTreeMap::new())),
                db,
            })
        } else {
            Err(anyhow!("Failed to initialize wallets db"))
        }
    }

    pub fn get_clients(&self) -> Result<MutexGuard<'_, BTreeMap<FederationId, ClientHandleArc>>> {
        if let Ok(clients) = self.clients.lock() {
            Ok(clients)
        } else {
            Err(anyhow!("Failed to get retrieve clients"))
        }
    }

    pub fn get_client_ids(&self) -> Result<Vec<FederationId>> {
        Ok(self.get_clients()?.clone().into_keys().collect())
    }

    pub fn get_client_by_id(&self, id: FederationId) -> Result<ClientHandleArc> {
        self.get_clients()?
            .get(&id)
            .cloned()
            .ok_or(anyhow!("Failed to get client handle with id {}", id))
    }

    pub async fn load_configs(&mut self) -> Result<()> {
        let mut dbtx = self.db.begin_transaction_nc().await;
        let configs = dbtx
            .find_by_prefix(&FederationIdKeyPrefix)
            .await
            .collect::<BTreeMap<FederationIdKey, FederationConfig>>()
            .await
            .values()
            .cloned()
            .collect::<Vec<_>>();

        let secret = self.mnemonic_secret().await?;

        for config in &configs {
            let id = config.invite_code.federation_id();
            if let Ok(wallet) = Wallet::from_opened(id, secret.clone()).await {
                self.get_clients()?.insert(id, wallet.client);
            }
        }

        Ok(())
    }

    pub async fn join(&mut self, invite_code: &str) -> Result<()> {
        let secret = self.mnemonic_secret().await?;
        let invite = InviteCode::from_str(invite_code)?;
        let wallet = Wallet::from_joined(&invite, secret).await?;
        let invite_code = InviteCode::from_str(invite_code)?;
        let config = FederationConfig { invite_code };
        let id = config.invite_code.federation_id();
        let mut dbtx = self.db.begin_transaction().await;

        dbtx.insert_entry(&FederationIdKey { id }, &config).await;
        dbtx.commit_tx_result().await?;

        self.get_clients()?
            .insert(config.invite_code.federation_id(), wallet.client);

        println!("{}", id.clone());

        Ok(())
    }

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
                Mnemonic::from_entropy(&entropy)?
            } else {
                let mnemonic = Bip39RootSecretStrategy::<12>::random(&mut thread_rng());
                Client::store_encodable_client_secret(&self.db, mnemonic.to_entropy()).await?;
                mnemonic
            },
        )
    }
}
