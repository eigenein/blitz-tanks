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

    /// Create a new sign-in URL for unit testing.
    #[cfg(test)]
    pub fn new_test() -> Self {
        Self::new("fake-id", "localhost")
    }
}
