use fedimint_client::{OperationId, RootSecret};
use fedimint_cursed_redb::MemAndRedb;
use futures::StreamExt;
use serde::Serialize;
use std::{str::FromStr, sync::Arc, time::Duration};

use crate::{
    database::FederationIdKey,
    types::{Error, Result, WalletError},
};
use fedimint_api_client::api::net::Connector;
use fedimint_client::{Client, ClientBuilder, ClientHandleArc};
use fedimint_core::{Amount, db::Database, invite_code::InviteCode};
use fedimint_mint_client::{
    MintClientInit, MintClientModule, OOBNotes, SelectNotesWithAtleastAmount,
};
use fedimint_wallet_client::WalletClientInit;

#[derive(Debug, Clone)]
pub struct Wallet {
    pub federation_id: FederationIdKey,
    client: ClientHandleArc,
    db: Database,
}

impl Wallet {
    async fn build() -> Result<ClientBuilder> {
        let mut builder = Client::builder()
            .await
            .map_err(|_| Error::Wallet(WalletError::BuildError))?
            .with_iroh_enable_next(true)
            .with_iroh_enable_dht(true);

        builder.with_connector(Connector::default());
        builder.with_module(MintClientInit);
        builder.with_module(WalletClientInit::default());

        Ok(builder)
    }

    async fn load_database(id: &str) -> Result<Database> {
        if let Some(db_dir) = dirs::data_local_dir() {
            let db_path = db_dir.join("tuimint/");
            let db_file = db_path.join(format!("{}.db", id));

            let _ = tokio::fs::create_dir_all(&db_path).await;
            let cursed_db = MemAndRedb::new(db_file)
                .await
                .map_err(|_| Error::DBLoadError)?;
            let db = Database::new(cursed_db, Default::default());

            Ok(db)
        } else {
            Err(Error::DBLoadError)
        }
    }

    pub async fn from_joined(invite_code: &str, secret: RootSecret) -> Result<Wallet> {
        let builder = Wallet::build().await?;

        let invite = InviteCode::from_str(invite_code).map_err(|_| Error::InvalidInviteCode)?;
        let db = Wallet::load_database(&invite.federation_id().to_string()).await?;
        let client = builder
            .preview(&invite)
            .await
            .map_err(|_| Error::Wallet(WalletError::PreviewError))?
            .join(db.clone(), secret)
            .await
            .map_err(|_| Error::Wallet(WalletError::JoinError))?;

        Ok(Wallet {
            federation_id: FederationIdKey {
                id: invite.federation_id(),
            },
            client: Arc::new(client),
            db: db.clone(),
        })
    }

    pub async fn from_opened(federation_id: FederationIdKey, secret: RootSecret) -> Result<Wallet> {
        let builder = Wallet::build().await?;
        let db = Wallet::load_database(&federation_id.id.to_string()).await?;
        let client = builder
            .open(db.clone(), secret)
            .await
            .map_err(|_| Error::Wallet(WalletError::OpenError))?;

        Ok(Wallet {
            federation_id,
            client: Arc::new(client),
            db: db.clone(),
        })
    }

    pub async fn balance(&mut self) -> Result<Amount> {
        if let Some(balance) = self.client.get_balance().await {
            Ok(balance)
        } else {
            Ok(Amount::from_msats(0))
        }
    }

    pub async fn spend_ecash(&mut self, amount: Amount) -> Result<(OperationId, OOBNotes)> {
        let mint = self
            .client
            .get_first_module::<MintClientModule>()
            .map_err(|_| Error::MissingModule)?;

        mint.spend_notes_with_selector(
            &SelectNotesWithAtleastAmount,
            amount,
            Duration::from_secs(60 * 60 * 24),
            true,
            NoMeta {},
        )
        .await
        .map_err(|_| Error::SpendError)
    }

    pub async fn receive_ecash(&mut self, notes: &str) -> Result<Amount> {
        let mint = self
            .client
            .get_first_module::<MintClientModule>()
            .map_err(|_| Error::MissingModule)?;

        let oob_notes = OOBNotes::from_str(notes).map_err(|_| Error::InvalidNotes)?;
        let operation_id = mint
            .reissue_external_notes(oob_notes.clone(), NoMeta {})
            .await
            .map_err(|_| Error::InvalidNotes)?;

        let mut updates = mint
            .subscribe_reissue_external_notes(operation_id)
            .await
            .map_err(|_| Error::InvalidNotes)?
            .into_stream();

        while let Some(update) = updates.next().await {
            if let fedimint_mint_client::ReissueExternalNotesState::Failed(_) = update {
                return Err(Error::InvalidNotes);
            }
        }

        Ok(oob_notes.total_amount())
    }
}

#[derive(Serialize)]
struct NoMeta {}
