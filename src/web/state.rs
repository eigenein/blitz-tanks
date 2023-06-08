use std::{collections::hash_map::RandomState, sync::Arc, time::Duration};

use moka::future::Cache;

#[cfg(test)]
use crate::prelude::Result;
use crate::{
    db::Db,
    weegee::{VehiclesStats, WeeGee},
};

#[derive(Clone)]
pub struct AppState {
    /// [Wargaming.net OpenID][1] sign-in URL.
    ///
    /// [1]: https://developers.wargaming.net/reference/all/wot/auth/login/
    pub sign_in_url: Arc<String>,

    pub db: Db,
    weegee: WeeGee,

    /// Account's vehicle's cache,
    /// used to check whether a certain user is allowed to rate a certain vehicle.
    vehicles_stats_cache: Cache<u32, Arc<VehiclesStats>, RandomState>,
}

impl AppState {
    pub fn new(
        db: Db,
        frontend_application_id: &str,
        backend_application_id: &str,
        domain_name: &str,
    ) -> Result<Self> {
        Ok(Self {
            db,
            weegee: WeeGee::new(backend_application_id)?,
            sign_in_url: Arc::new(format!(
                "https://api.worldoftanks.eu/wot/auth/login/?application_id={frontend_application_id}&redirect_uri=//{domain_name}/authenticate"
            )),
            vehicles_stats_cache: Cache::builder()
                .time_to_idle(Duration::from_secs(600))
                .build(),
        })
    }

    #[cfg(test)]
    pub fn new_test() -> Result<Self> {
        Self::new(Db::open_temporary()?, "test", "test", "localhost:8080")
    }
}
