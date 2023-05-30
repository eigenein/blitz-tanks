use axum::extract::{FromRef, FromRequestParts};
use axum::headers::Cookie;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::{async_trait, RequestPartsExt, TypedHeader};
use uuid::Uuid;

use crate::db::Db;
use crate::web::prelude::*;

/// Client-side session.
pub enum Session {
    Authenticated {
        /// Session ID.
        id: Uuid,

        /// Authenticated user.
        user: crate::db::sessions::User,
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
        let session_id = cookie
            .as_ref()
            .and_then(|cookie| cookie.get(Self::SESSION_COOKIE_NAME));
        match session_id {
            Some(session_id) => {
                let sessions = Db::from_ref(state).sessions()?;
                unimplemented!()
            }
            None => Ok(Session::Anonymous),
        }
    }
}
