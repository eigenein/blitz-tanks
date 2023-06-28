use std::{collections::HashMap, sync::Arc, time::Duration};

use futures::TryStreamExt;
use indexmap::IndexMap;
use itertools::Itertools;
use moka::future::Cache;
use tokio::task::spawn_blocking;
use tracing::warn;

use crate::{
    db::{sessions::Sessions, votes::Votes, Db},
    models::{RatedTankId, Vehicle},
    prelude::*,
    trainer::item_item::Model,
    wg::{VehicleStats, Wg},
};

#[derive(Clone)]
pub struct AppState {
    pub sign_in_url: Arc<String>,

    pub wg: Wg,
    pub tankopedia: Arc<HashMap<u16, Vehicle>>,
    pub model: Arc<Model>,

    pub session_manager: Sessions,
    pub vote_manager: Votes,

    stats_cache: Cache<u32, Arc<IndexMap<u16, VehicleStats>>>,

    #[allow(clippy::type_complexity)]
    predictions_cache: Cache<u32, Arc<Box<[RatedTankId]>>>,
}

impl AppState {
    pub async fn new(db: &Db, application_id: &str, wg: Wg, public_address: &str) -> Result<Self> {
        let tankopedia = Arc::new(db.tankopedia().await?.load().await?);
        if tankopedia.is_empty() {
            warn!("⚠️ Tankopedia database is empty, please re-run with `--update-tankopedia`");
        }

        let sign_in_url = Arc::new(format!(
            "https://api.worldoftanks.eu/wot/auth/login/?application_id={application_id}&redirect_uri=//{public_address}/welcome"
        ));
        let stats_cache = Cache::builder()
            .max_capacity(1000)
            .time_to_idle(Duration::from_secs(300))
            .build();
        let predictions_cache = Cache::builder()
            .max_capacity(1000)
            .time_to_idle(Duration::from_secs(300))
            .build();

        // TODO: model auto-reloader.
        #[cfg(not(test))]
        let model = db.models().await?.get_latest().await?;
        #[cfg(test)]
        let model = Model::empty();

        Ok(Self {
            sign_in_url,
            wg,
            tankopedia,
            model: Arc::new(model),
            session_manager: db.sessions().await?,
            vote_manager: db.votes().await?,
            stats_cache,
            predictions_cache,
        })
    }

    #[cfg(test)]
    pub async fn new_test() -> Result<Self> {
        Self::new(&Db::open_unittests().await?, "test", Wg::new("test")?, "localhost:8080").await
    }

    /// Retrieve the account's vehicle's statistics and cache it.
    #[instrument(skip_all, fields(account_id = account_id))]
    pub async fn get_vehicle_stats(
        &self,
        account_id: u32,
    ) -> Result<Arc<IndexMap<u16, VehicleStats>>> {
        self.stats_cache
            .try_get_with(account_id, async {
                let map = self
                    .wg
                    .get_vehicles_stats(account_id)
                    .await?
                    .into_iter()
                    .filter(VehicleStats::is_played)
                    .sorted_unstable_by(|lhs, rhs| rhs.last_battle_time.cmp(&lhs.last_battle_time))
                    .map(|stats| (stats.tank_id, stats))
                    .collect();
                Ok(Arc::new(map))
            })
            .await
            .map_err(|error: Arc<Error>| anyhow!(error))
            .with_context(|| format!("failed to retrieve account {account_id}'s vehicles stats"))
    }

    #[instrument(skip_all, fields(account_id = account_id, tank_id = tank_id))]
    pub async fn owns_vehicle(&self, account_id: u32, tank_id: u16) -> Result<bool> {
        Ok(self
            .get_vehicle_stats(account_id)
            .await?
            .get(&tank_id)
            .is_some_and(VehicleStats::is_played))
    }

    #[instrument(skip_all, fields(account_id))]
    pub async fn get_predictions(&self, account_id: u32) -> Result<Arc<Box<[RatedTankId]>>> {
        let model = self.model.clone();
        let stats = self.get_vehicle_stats(account_id).await?;

        self.predictions_cache
            .try_get_with(account_id, async {
                let source_ratings = self
                    .vote_manager
                    .iter_by_account_id(account_id)
                    .await?
                    .map_ok(|vote| (vote.tank_id, vote.rating))
                    .try_collect()
                    .await?;
                let target_ids: Vec<u16> = self
                    .tankopedia
                    .keys()
                    .filter(move |tank_id| !stats.contains_key(*tank_id))
                    .copied()
                    .collect();
                let predict = move || {
                    model
                        .predict_many(target_ids, &source_ratings)
                        .sorted_unstable()
                        .take_while(RatedTankId::is_positive)
                        .collect()
                };
                spawn_blocking(predict).await.map(Arc::new).map(Ok)?
            })
            .await
            .map_err(|error: Arc<Error>| anyhow!(error))
            .with_context(|| format!("failed to generate recommendations for #{account_id}"))
    }

    /// Remove predictions for the account from the cache, if any.
    pub async fn purge_predictions(&self, account_id: u32) {
        self.predictions_cache.remove(&account_id).await;
    }
}
