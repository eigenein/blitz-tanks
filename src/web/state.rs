use std::{sync::Arc, time::Duration};

use arc_swap::ArcSwap;
use futures::TryStreamExt;
use indexmap::IndexMap;
use itertools::Itertools;
use moka::future::Cache;
use tokio::task::spawn_blocking;

use crate::{
    db::{sessions::Sessions, votes::Votes, Db},
    models::{AccountId, RatedTankId, TankId},
    prelude::*,
    tankopedia::vendored::TANKOPEDIA,
    trainer::item_item::Model,
    wg::{VehicleStats, Wg},
};

#[derive(Clone)]
pub struct AppState {
    pub sign_in_url: Arc<String>,

    pub wg: Wg,
    pub model: Arc<ArcSwap<Model>>,

    pub session_manager: Sessions,
    pub vote_manager: Votes,

    stats_cache: Cache<AccountId, Arc<IndexMap<TankId, VehicleStats>>>,

    #[allow(clippy::type_complexity)]
    predictions_cache: Cache<AccountId, Arc<Vec<RatedTankId>>>,
}

impl AppState {
    pub async fn new(db: &Db, application_id: &str, wg: Wg, public_address: &str) -> Result<Self> {
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

        #[cfg(not(test))]
        let model = db.models().await?.get_latest().await?;
        #[cfg(test)]
        let model = Model::empty();

        Ok(Self {
            sign_in_url,
            wg,
            model: Arc::new(ArcSwap::new(Arc::new(model))),
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
    #[instrument(skip_all, fields(account_id = %account_id))]
    pub async fn get_vehicles_stats(
        &self,
        account_id: AccountId,
    ) -> Result<Arc<IndexMap<TankId, VehicleStats>>> {
        self.stats_cache
            .try_get_with(account_id, async {
                let map = self
                    .wg
                    .get_vehicles_stats(account_id)
                    .await?
                    .into_iter()
                    .filter(VehicleStats::is_played)
                    .filter(|stats| TANKOPEDIA.contains_key(&u16::from(stats.tank_id)))
                    .sorted_unstable_by(|lhs, rhs| rhs.last_battle_time.cmp(&lhs.last_battle_time))
                    .map(|stats| (stats.tank_id, stats))
                    .collect();
                Ok(Arc::new(map))
            })
            .await
            .map_err(|error: Arc<Error>| anyhow!(error))
            .with_context(|| format!("failed to retrieve account {account_id}'s vehicles stats"))
    }

    #[instrument(skip_all, fields(account_id = %account_id, tank_id = %tank_id))]
    pub async fn owns_vehicle(&self, account_id: AccountId, tank_id: TankId) -> Result<bool> {
        Ok(self
            .get_vehicles_stats(account_id)
            .await?
            .get(&tank_id)
            .is_some_and(VehicleStats::is_played))
    }

    #[instrument(skip_all, fields(account_id = %account_id))]
    pub async fn get_predictions(&self, account_id: AccountId) -> Result<Arc<Vec<RatedTankId>>> {
        let model = self.model.clone();
        let stats = self.get_vehicles_stats(account_id).await?;

        self.predictions_cache
            .try_get_with(account_id, async {
                let source_ratings = self
                    .vote_manager
                    .iter_by_account_id(account_id)
                    .await?
                    .map_ok(|vote| (vote.id.tank_id, vote.rating))
                    .try_collect()
                    .await?;
                let target_ids: Vec<TankId> = TANKOPEDIA
                    .keys()
                    .copied()
                    .map(TankId::from)
                    .filter(move |tank_id| !stats.contains_key(tank_id))
                    .collect();
                let predict = move || {
                    ArcSwap::load(&model)
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
    #[instrument(skip_all, fields(account_id = %account_id))]
    pub async fn purge_predictions(&self, account_id: AccountId) {
        self.predictions_cache.remove(&account_id).await;
    }
}
