use std::{sync::Arc, time::Duration};

use anyhow::Context;
use indexmap::IndexMap;
use itertools::Itertools;
use moka::future::Cache;
use tracing::instrument;

use crate::{
    prelude::*,
    wg::{VehicleStats, Wg},
};

/// Proxy for user's vehicles' statistics.
#[derive(Clone)]
pub struct VehicleStatsGetter {
    wg: Wg,
    cache: Cache<u32, Arc<IndexMap<u16, VehicleStats>>>,
}

impl From<Wg> for VehicleStatsGetter {
    fn from(wg: Wg) -> Self {
        Self {
            wg,
            cache: Cache::builder()
                .max_capacity(1000)
                .time_to_idle(Duration::from_secs(300))
                .build(),
        }
    }
}

impl VehicleStatsGetter {
    /// Retrieve the account's vehicle's statistics and cache it.
    #[instrument(skip_all, fields(account_id = account_id))]
    pub async fn get(&self, account_id: u32) -> Result<Arc<IndexMap<u16, VehicleStats>>> {
        self.cache
            .try_get_with(account_id, async {
                let map = self
                    .wg
                    .get_vehicles_stats(account_id)
                    .await?
                    .into_iter()
                    .filter(VehicleStats::is_played)
                    .sorted_unstable_by_key(|stats| -stats.last_battle_time)
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
        Ok(self.get(account_id).await?.get(&tank_id).is_some_and(VehicleStats::is_played))
    }
}
