use axum::extract::State;

use crate::web::partials::*;
use crate::web::prelude::*;
use crate::web::state::*;

pub async fn get(State(SignInUrl(sign_in_url)): State<SignInUrl>) -> Markup {
    html! {
        (head())
        body {
            section.hero.is-fullheight {
                div.hero-head {
                    (navbar())
                }

                div.hero-body {
                    div.container {
                        div.columns {
                            div.column.is-half-widescreen.is-offset-one-quarter-widescreen {
                                p.title.has-text-weight-light {
                                    "Rate "
                                    span.has-text-weight-medium { "World of Tanks Blitz" }
                                    " vehicles"
                                }
                                p.subtitle.has-text-weight-light {
                                    "Get "
                                    span.has-text-weight-medium { "personal" }
                                    " recommendations"
                                }
                                form {
                                    div.field {
                                        div.control {
                                            a.button.is-warning.is-large.is-responsive."px-6" href=(sign_in_url) {
                                                span.icon { i.fa-solid.fa-right-to-bracket {} }
                                                strong { "Sign in" }
                                            }
                                        }
                                        p.help {
                                            "with your "
                                            a href="https://wargaming.net/personal/" { "Wargaming.net ID" }
                                            " (only Europe)"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            ((footer()))
        }
    }
}
