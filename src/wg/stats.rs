use std::{sync::Arc, time::Duration};

use anyhow::Context;
use chrono::LocalResult;
use indexmap::IndexMap;
use itertools::Itertools;
use moka::future::Cache;
use serde::Deserialize;
use tracing::instrument;

use crate::{prelude::*, wg::Wg};

/// Partial user's vehicle statistics.
#[derive(Deserialize)]
pub struct VehicleStats {
    pub tank_id: i32,
    pub last_battle_time: i64,

    #[serde(rename = "all")]
    pub inner: InnerVehicleStats,
}

impl VehicleStats {
    pub const FAKE_NON_PLAYED: Self = Self {
        tank_id: 2,
        last_battle_time: 0,
        inner: InnerVehicleStats { n_battles: 0 },
    };
    pub const FAKE_PLAYED: Self = Self {
        tank_id: 1,
        last_battle_time: 0,
        inner: InnerVehicleStats { n_battles: 1 },
    };

    pub fn last_battle_time(&self) -> LocalResult<DateTime> {
        Utc.timestamp_opt(self.last_battle_time, 0)
    }

    pub const fn is_played(&self) -> bool {
        self.inner.n_battles != 0
    }
}

#[derive(Deserialize)]
pub struct InnerVehicleStats {
    #[serde(rename = "battles")]
    pub n_battles: u32,
}

/// Proxy for user's vehicles' statistics.
#[derive(Clone)]
pub struct VehicleStatsGetter {
    wee_gee: Wg,
    cache: Cache<u32, Arc<IndexMap<i32, VehicleStats>>>,
}

impl From<Wg> for VehicleStatsGetter {
    fn from(wee_gee: Wg) -> Self {
        Self {
            wee_gee,
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
    pub async fn get(&self, account_id: u32) -> Result<Arc<IndexMap<i32, VehicleStats>>> {
        self.cache
            .try_get_with(account_id, async {
                let map = self
                    .wee_gee
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
    pub async fn owns_vehicle(&self, account_id: u32, tank_id: i32) -> Result<bool> {
        Ok(self.get(account_id).await?.get(&tank_id).is_some_and(VehicleStats::is_played))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::wg::result::WgResult;

    #[test]
    fn vehicles_stats_ok() -> Result {
        serde_json::from_str::<WgResult<HashMap<String, Vec<VehicleStats>>>>(
            // language=json
            r#"{"status":"ok","meta":{"count":1},"data":{"594778041":[{"all":{"battles":248},"last_battle_time":1681146251,"tank_id":18769}]}}"#,
        )?;
        Ok(())
    }
}
