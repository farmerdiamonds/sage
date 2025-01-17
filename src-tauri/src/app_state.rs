use std::{
    ops::{Deref, DerefMut},
    path::Path,
    sync::Arc,
};

use sage::Sage;
use sage_api::SyncEvent as ApiEvent;
use sage_wallet::SyncEvent;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;

use crate::error::Result;

pub type AppState = Arc<Mutex<AppStateInner>>;

pub struct AppStateInner {
    pub app_handle: AppHandle,
    pub initialized: bool,
    pub sage: Sage,
}

impl Deref for AppStateInner {
    type Target = Sage;

    fn deref(&self) -> &Self::Target {
        &self.sage
    }
}

impl DerefMut for AppStateInner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.sage
    }
}

impl AppStateInner {
    pub fn new(app_handle: AppHandle, path: &Path) -> Self {
        Self {
            app_handle,
            initialized: false,
            sage: Sage::new(path),
        }
    }

    pub async fn initialize(&mut self) -> Result<bool> {
        if self.initialized {
            return Ok(true);
        }

        self.initialized = true;

        let mut receiver = self.sage.initialize().await?;
        let app_handle = self.app_handle.clone();

        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                let event = match event {
                    SyncEvent::Start(ip) => ApiEvent::Start { ip: ip.to_string() },
                    SyncEvent::Stop => ApiEvent::Stop,
                    SyncEvent::Subscribed => ApiEvent::Subscribed,
                    SyncEvent::DerivationIndex { .. } => ApiEvent::Derivation,
                    // TODO: New event?
                    SyncEvent::CoinsUpdated { .. }
                    | SyncEvent::TransactionUpdated { .. }
                    | SyncEvent::TransactionEnded { .. }
                    | SyncEvent::OfferUpdated { .. } => ApiEvent::CoinState,
                    SyncEvent::PuzzleBatchSynced => ApiEvent::PuzzleBatchSynced,
                    SyncEvent::CatInfo => ApiEvent::CatInfo,
                    SyncEvent::DidInfo => ApiEvent::DidInfo,
                    SyncEvent::NftData => ApiEvent::NftData,
                };
                if app_handle.emit("sync-event", event).is_err() {
                    break;
                }
            }

            Result::Ok(())
        });

        Ok(false)
    }
}
