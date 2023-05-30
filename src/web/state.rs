#[derive(Clone)]
pub struct SignInUrl(pub String);

impl SignInUrl {
    pub fn new(application_id: &str, domain_name: &str) -> Self {
        Self(format!(
            "https://api.worldoftanks.eu/wot/auth/login/?application_id={application_id}&redirect_uri=//{domain_name}/authenticate"
        ))
    }
}
