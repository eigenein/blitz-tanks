use std::{collections::HashMap, sync::Arc};

use tracing::warn;

use crate::{
    db::{sessions::Sessions, votes::Votes, Db},
    models::vehicle::Vehicle,
    prelude::*,
    wg::{stats::VehicleStatsGetter, Wg},
};

#[derive(Clone)]
pub struct AppState {
    pub sign_in_url: Arc<String>,

    pub wg: Wg,
    pub tankopedia: Arc<HashMap<i32, Vehicle>>,
    pub vehicle_stats_getter: VehicleStatsGetter,

    pub session_manager: Sessions,
    pub vote_manager: Votes,
}

impl AppState {
    pub async fn new(
        db: &Db,
        frontend_application_id: &str,
        wee_gee: Wg,
        public_address: &str,
    ) -> Result<Self> {
        let tankopedia = Arc::new(db.tankopedia_manager().await?.load().await?);
        if tankopedia.is_empty() {
            warn!("⚠️ Tankopedia database is empty, please re-run with `--update-tankopedia`");
        }

        Ok(Self {
            sign_in_url: Arc::new(format!(
                "https://api.worldoftanks.eu/wot/auth/login/?application_id={frontend_application_id}&redirect_uri=//{public_address}/welcome"
            )),

            wg: wee_gee.clone(),
            tankopedia,
            vehicle_stats_getter: VehicleStatsGetter::from(wee_gee),

            session_manager: db.session_manager().await?,
            vote_manager: db.vote_manager().await?,
        })
    }

    #[cfg(test)]
    pub async fn new_test() -> Result<Self> {
        Self::new(&Db::open_unittests().await?, "test", Wg::new("test")?, "localhost:8080").await
    }
}
