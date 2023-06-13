use std::{collections::HashMap, sync::Arc};

use tracing::warn;

use crate::{
    db::{Db, SessionManager, VoteManager},
    models::VehicleDescription,
    prelude::*,
    weegee::{VehicleStatsGetter, WeeGee},
};

#[derive(Clone)]
pub struct AppState {
    pub sign_in_url: Arc<String>,

    pub wee_gee: WeeGee,
    pub tankopedia: Arc<HashMap<u16, VehicleDescription>>,
    pub vehicle_stats_getter: VehicleStatsGetter,

    pub session_manager: SessionManager,
    pub vote_manager: VoteManager,
}

impl AppState {
    pub fn new(
        db: &Db,
        frontend_application_id: &str,
        wee_gee: WeeGee,
        public_address: &str,
    ) -> Result<Self> {
        let tankopedia = Arc::new(db.tankopedia_manager()?.load()?);
        if tankopedia.is_empty() {
            warn!("⚠️ tankopedia database is empty, please re-run with `--update-tankopedia`");
        }

        Ok(Self {
            sign_in_url: Arc::new(format!(
                "https://api.worldoftanks.eu/wot/auth/login/?application_id={frontend_application_id}&redirect_uri=//{public_address}/welcome"
            )),

            wee_gee: wee_gee.clone(),
            tankopedia,
            vehicle_stats_getter: VehicleStatsGetter::from(wee_gee),

            session_manager: db.session_manager()?,
            vote_manager: db.vote_manager()?,
        })
    }

    #[cfg(test)]
    pub async fn new_test() -> Result<Self> {
        Self::new(&Db::open_temporary().await?, "test", WeeGee::new("test")?, "localhost:8080")
    }
}
