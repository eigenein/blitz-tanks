use std::{borrow::Cow, io::stderr};

use clap::crate_version;
use sentry::{
    integrations::{anyhow::capture_anyhow, tracing::EventFilter},
    ClientInitGuard, ClientOptions, Scope,
};
use tracing::{error, info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

use crate::{models::User, prelude::*};

pub fn init(sentry_dsn: Option<String>, traces_sample_rate: f32) -> Result<ClientInitGuard> {
    let sentry_options = ClientOptions {
        release: Some(Cow::Borrowed(crate_version!())),
        traces_sample_rate,
        enable_profiling: false, // FIXME: causes the bug in `tracing`.
        profiles_sample_rate: traces_sample_rate,
        attach_stacktrace: true,
        send_default_pii: true,
        in_app_include: vec!["blitz_tanks"],
        ..Default::default()
    };
    let guard = sentry::init((sentry_dsn, sentry_options));

    let sentry_layer = sentry::integrations::tracing::layer()
        .event_filter(|_metadata| EventFilter::Breadcrumb)
        .span_filter(|metadata| metadata.level() >= &Level::DEBUG);

    let format_filter = EnvFilter::try_from_env("BLITZ_TANKS_LOG")
        .or_else(|_| EnvFilter::try_new("warn,blitz_tanks=info"))?;
    let format_layer = tracing_subscriber::fmt::layer()
        .with_writer(stderr)
        .without_time()
        .with_filter(format_filter);

    tracing_subscriber::Registry::default()
        .with(sentry_layer)
        .with(format_layer)
        .try_init()?;

    info!(is_sentry_enabled = guard.is_enabled(), "🥅");
    Ok(guard)
}

pub fn configure_user(scope: &mut Scope, user: Option<&User>) {
    match user {
        Some(user) => {
            scope.set_tag("user.is_anonymous", false);
            scope.set_user(Some(sentry::User {
                id: Some(user.account_id.to_string()),
                username: Some(user.nickname.clone()),
                ..Default::default()
            }));
        }
        None => {
            scope.set_tag("user.is_anonymous", true);
            scope.set_user(None);
        }
    }
}

/// Proxy the result, and report a possible error.
#[inline]
pub fn trace<T>(result: Result<T>) -> Result<T> {
    if let Err(error) = &result {
        let event_id = capture_anyhow(error);
        error!(?event_id, "💥 Failed: {:#}", error);
    }
    result
}
