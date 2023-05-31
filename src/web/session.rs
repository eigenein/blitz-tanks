use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    headers::Cookie,
    http::{request::Parts, StatusCode},
    RequestPartsExt, TypedHeader,
};
use uuid::Uuid;

use crate::{db::Db, web::prelude::*};

/// Client-side session.
pub enum Session {
    Authenticated {
        /// Session ID.
        id: Uuid,

        /// Authenticated user.
        user: crate::db::session::User,
    },

    /// Unidentified user: the session cookie is either missing or invalid.
    Anonymous,
}

impl Session {
    pub const SESSION_COOKIE_NAME: &'static str = "blitzTanksSessionId";
}

#[async_trait]
impl<S> FromRequestParts<S> for Session
where
    Db: FromRef<S>,
    S: Sync,
{
    type Rejection = WebError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> WebResult<Self> {
        let cookie: Option<TypedHeader<Cookie>> = parts.extract().await?;
        let Some(cookie) = cookie else { return Ok(Session::Anonymous) };
        match cookie.get(Self::SESSION_COOKIE_NAME) {
            Some(session_id) => {
                let manager = Db::from_ref(state).session_manager()?;
                unimplemented!()
            }
            None => Ok(Session::Anonymous),
        }
    }
}
