use std::sync::Arc;

#[cfg(test)]
use crate::prelude::Result;
use crate::{db::Db, weegee::WeeGee};

#[derive(Clone)]
pub struct AppState {
    /// [Wargaming.net OpenID][1] sign-in URL.
    ///
    /// [1]: https://developers.wargaming.net/reference/all/wot/auth/login/
    pub sign_in_url: Arc<String>,

    pub db: Db,
    pub wg: WeeGee,
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
            wg: WeeGee::new(backend_application_id)?,
            sign_in_url: Arc::new(format!(
                "https://api.worldoftanks.eu/wot/auth/login/?application_id={frontend_application_id}&redirect_uri=//{domain_name}/authenticate"
            )),
        })
    }

    #[cfg(test)]
    pub fn new_test() -> Result<Self> {
        Self::new(Db::open_temporary()?, "test", "test", "localhost:8080")
    }
}
