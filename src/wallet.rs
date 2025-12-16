use fedimint_bip39::{Bip39RootSecretStrategy, Mnemonic};
use fedimint_client::{
    OperationId, RootSecret, module::oplog::UpdateStreamOrOutcome, secret::RootSecretStrategy,
};
use futures::StreamExt;
use serde::Serialize;
use std::{path::PathBuf, str::FromStr, sync::Arc, time::Duration};

use fedimint_api_client::api::net::Connector;
use fedimint_client::{Client, ClientBuilder, ClientHandleArc};
use fedimint_core::{Amount, db::Database, invite_code::InviteCode, util::NextOrPending};
use fedimint_cursed_redb::MemAndRedb;
use fedimint_mint_client::{
    MintClientInit, MintClientModule, OOBNotes, SelectNotesWithAtleastAmount,
};
use fedimint_wallet_client::WalletClientInit;
use rand::thread_rng;

#[derive(Debug)]
pub enum WalletError {
    BuildError,
    DBLoadError,
    JoinError,
    OpenError,
    InvalidInvite,
    PreviewError,
    MnemonicError,
    NoClient,
    NoDb,
    MissingModule,
    SpendError,
    InvalidNotes,
}

#[derive(Debug, Clone)]
pub struct Wallet {
    client: Option<ClientHandleArc>,
    db: Option<Database>,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            client: None,
            db: None,
        }
    }

    fn get_client(&self) -> Result<ClientHandleArc, WalletError> {
        if let Some(client) = &self.client {
            Ok(client.clone())
        } else {
            Err(WalletError::NoClient)
        }
    }

    async fn build(&self) -> Result<ClientBuilder, WalletError> {
        let mut builder = Client::builder()
            .await
            .map_err(|_| WalletError::BuildError)?
            .with_iroh_enable_next(true)
            .with_iroh_enable_dht(true);

        builder.with_connector(Connector::default());
        builder.with_module(MintClientInit);
        builder.with_module(WalletClientInit::default());

        Ok(builder)
    }

    async fn load_database(&mut self) -> Result<Database, WalletError> {
        // TODO: store in ~/.config
        let db_path = PathBuf::from("./tuimint-db");
        let db_file = db_path.join("tuimint.db");

        let _ = tokio::fs::create_dir_all(&db_path).await;
        let cursed_db = MemAndRedb::new(db_file)
            .await
            .map_err(|_| WalletError::DBLoadError)?;
        let db = Database::new(cursed_db, Default::default());

        self.db = Some(db.clone());

        Ok(db)
    }

    async fn load_or_generate_mnemonic(&self) -> Result<Mnemonic, WalletError> {
        if let Some(db) = &self.db {
            Ok(
                if let Ok(entropy) = Client::load_decodable_client_secret::<Vec<u8>>(db).await {
                    Mnemonic::from_entropy(&entropy).map_err(|_| WalletError::MnemonicError)?
                } else {
                    let mnemonic = Bip39RootSecretStrategy::<12>::random(&mut thread_rng());
                    Client::store_encodable_client_secret(db, mnemonic.to_entropy())
                        .await
                        .map_err(|_| WalletError::MnemonicError)?;
                    mnemonic
                },
            )
        } else {
            Err(WalletError::NoDb)
        }
    }

    pub async fn join(&mut self, invite_code: &str) -> Result<&mut Self, WalletError> {
        let db = self.load_database().await?;
        let builder = self.build().await?;

        let mnemonic = self.load_or_generate_mnemonic().await?;
        let secret = RootSecret::StandardDoubleDerive(
            Bip39RootSecretStrategy::<12>::to_root_secret(&mnemonic),
        );
        let invite = InviteCode::from_str(invite_code).map_err(|_| WalletError::InvalidInvite)?;
        let client = builder
            .preview(&invite)
            .await
            .map_err(|_| WalletError::PreviewError)?
            .join(db, secret)
            .await
            .map_err(|_| WalletError::JoinError)
            .map(Arc::new)?;

        self.client = Some(client);

        Ok(self)
    }

    pub async fn open(&mut self) -> Result<&mut Self, WalletError> {
        let db = self.load_database().await?;
        let builder = self.build().await?;

        let mnemonic = self.load_or_generate_mnemonic().await?;
        let secret = RootSecret::StandardDoubleDerive(
            Bip39RootSecretStrategy::<12>::to_root_secret(&mnemonic),
        );
        let client = builder
            .open(db, secret)
            .await
            .map_err(|_| WalletError::OpenError)
            .map(Arc::new)?;

        self.client = Some(client);

        Ok(self)
    }

    pub async fn balance(&mut self) -> Result<Amount, WalletError> {
        let client = self.get_client()?;

        if let Some(balance) = client.get_balance().await {
            Ok(balance)
        } else {
            Ok(Amount::from_msats(0))
        }
    }

    pub async fn spend_ecash(
        &mut self,
        amount: Amount,
    ) -> Result<(OperationId, OOBNotes), WalletError> {
        let client = self.get_client()?;
        let mint = client
            .get_first_module::<MintClientModule>()
            .map_err(|_| WalletError::MissingModule)?;

        mint.spend_notes_with_selector(
            &SelectNotesWithAtleastAmount,
            amount,
            Duration::from_secs(60 * 60 * 24),
            true,
            NoMeta {},
        )
        .await
        .map_err(|_| WalletError::SpendError)
    }

    pub async fn receive_ecash(&mut self, notes: &str) -> Result<Amount, WalletError> {
        let client = self.get_client()?;
        let mint = client
            .get_first_module::<MintClientModule>()
            .map_err(|_| WalletError::MissingModule)?;

        let oob_notes = OOBNotes::from_str(notes).map_err(|_| WalletError::InvalidNotes)?;
        let operation_id = mint
            .reissue_external_notes(oob_notes.clone(), NoMeta {})
            .await
            .map_err(|_| WalletError::InvalidNotes)?;

        let mut updates = mint
            .subscribe_reissue_external_notes(operation_id)
            .await
            .map_err(|_| WalletError::InvalidNotes)?
            .into_stream();

        while let Some(update) = updates.next().await {
            if let fedimint_mint_client::ReissueExternalNotesState::Failed(_) = update {
                return Err(WalletError::InvalidNotes);
            }
        }

        Ok(oob_notes.total_amount())
    }
}

#[derive(Serialize)]
struct NoMeta {}
