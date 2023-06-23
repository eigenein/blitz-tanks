use crate::web::error::WebError;

pub type WebResult<T> = Result<T, WebError>;
