use std::{collections::HashMap, sync::Arc};

use tracing::warn;

use crate::{
    db::{Db, SessionManager},
    models::VehicleDescription,
    prelude::*,
    weegee::{VehicleStatsGetter, WeeGee},
};

#[derive(Clone)]
pub struct AppState {
    pub sign_in_url: Arc<String>,
    pub tankopedia: Arc<HashMap<u16, VehicleDescription>>,
    pub session_manager: SessionManager,
    pub vehicle_stats_getter: VehicleStatsGetter,
    pub wee_gee: WeeGee,
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
            tankopedia,
            session_manager: db.session_manager()?,
            sign_in_url: Arc::new(format!(
                "https://api.worldoftanks.eu/wot/auth/login/?application_id={frontend_application_id}&redirect_uri=//{public_address}/welcome"
            )),
            vehicle_stats_getter: VehicleStatsGetter::from(wee_gee.clone()),
            wee_gee,
        })
    }

    #[cfg(test)]
    pub fn new_test() -> Result<Self> {
        Self::new(&Db::open_temporary()?, "test", WeeGee::new("test")?, "localhost:8080")
    }
}
