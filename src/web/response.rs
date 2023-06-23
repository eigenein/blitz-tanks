use axum::response::{IntoResponse, Redirect, Response};

use crate::web::prelude::*;

pub enum OptionalRedirect {
    Markup(Markup),
    Redirect(Redirect),
}

impl IntoResponse for OptionalRedirect {
    fn into_response(self) -> Response {
        match self {
            Self::Markup(markup) => markup.into_response(),
            Self::Redirect(redirect) => redirect.into_response(),
        }
    }
}
