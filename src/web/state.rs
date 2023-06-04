use axum::extract::FromRef;

use crate::{db::Db, prelude::*};

#[derive(Clone)]
pub struct AppState {
    pub sign_in_url: SignInUrl,
    pub db: Db,
}

impl AppState {
    pub fn new(db: Db, application_id: &str, domain_name: &str) -> Self {
        Self {
            db,
            sign_in_url: SignInUrl::new(application_id, domain_name),
        }
    }

    #[cfg(test)]
    pub fn new_test() -> Result<Self> {
        Ok(Self::new(Db::open_temporary()?, "test", "localhost:8080"))
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
