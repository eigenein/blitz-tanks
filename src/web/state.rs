use axum::extract::FromRef;

#[cfg(test)]
use crate::prelude::Result;
use crate::{db::Db, weegee::WeeGee};

#[derive(Clone)]
pub struct AppState {
    pub sign_in_url: SignInUrl,
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
            sign_in_url: SignInUrl::new(frontend_application_id, domain_name),
        })
    }

    #[cfg(test)]
    pub fn new_test() -> Result<Self> {
        Self::new(Db::open_temporary()?, "test", "test", "localhost:8080")
    }
}

/// [Wargaming.net OpenID][1] sign-in URL.
///
/// [1]: https://developers.wargaming.net/reference/all/wot/auth/login/
#[derive(Clone)]
pub struct SignInUrl(pub String);

impl SignInUrl {
    pub fn new(application_id: &str, domain_name: &str) -> Self {
        Self(format!(
            "https://api.worldoftanks.eu/wot/auth/login/?application_id={application_id}&redirect_uri=//{domain_name}/authenticate"
        ))
    }
}

impl FromRef<AppState> for SignInUrl {
    fn from_ref(input: &AppState) -> SignInUrl {
        input.sign_in_url.clone()
    }
}

impl FromRef<AppState> for Db {
    fn from_ref(input: &AppState) -> Self {
        input.db.clone()
    }
}
