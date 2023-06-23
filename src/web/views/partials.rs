use chrono::LocalResult;
use chrono_humanize::HumanTime;
use clap::crate_version;
use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::{
    models::{rating::Rating, user::User, vehicle::Vehicle},
    wg::VehicleStats,
};

pub fn head() -> Markup {
    html! {
        head {
            (DOCTYPE)
            meta name="viewport" content="width=device-width, initial-scale=1";
            meta charset="utf-8";
            title { "Recommender for World of Tanks Blitz" }
            link rel="shortcut icon" href="/favicon.ico" sizes="16x16 32x32";
            link rel="apple-touch-icon" type="image/png" href="/apple-touch-icon.png" sizes="180x180";
            link rel="icon" type="image/png" sizes="192x192" href="/icon-192.png";
            link rel="icon" type="image/png" sizes="512x512" href="/icon-512.png";
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/bulma/0.9.4/css/bulma.min.css" integrity="sha512-HqxHUkJM0SYcbvxUw5P60SzdOTy/QVwA1JJrvaXJv4q7lmbDZCmZaqz01UPOaQveoxfYRv1tHozWGPMcuTBuvQ==" crossorigin="anonymous" referrerpolicy="no-referrer";
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/bulma-prefers-dark/0.1.0-beta.1/bulma-prefers-dark.min.css" integrity="sha512-8L9NjgWBr9opkijcN9ZZCzzl7T3hVqji0baeKdTvfq1VN119XV4RNCGGI6vAF8ygQkSK0Qew84toTqqpzmbxUw==" crossorigin="anonymous" referrerpolicy="no-referrer";
            link rel="stylesheet" href=(concat!("/bulma-patches.css/", crate_version!()));
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" integrity="sha512-iecdLmaskl7CVkqkXNQ/ZH/XLlvWZOJyj7Yy7tcenmpD1ypASozpmT/E0iPtmFIB46ZmdtAc9eNBvH0H/ZpiBw==" crossorigin="anonymous" referrerpolicy="no-referrer";
            script src="https://js.sentry-cdn.com/e0851235b23f439aa285aa8f0b431547.min.js" crossorigin="anonymous" {}
            script src="https://cdnjs.cloudflare.com/ajax/libs/htmx/1.9.2/htmx.min.js" integrity="sha512-ULbUWm8wCS6zRoxK/2v51vUHGhKvK8PSiqA02tyUYlYoeQm5wB8xr8lObq5zmNGpYaZsED0NLhaiPAAm2VbhXw==" crossorigin="anonymous" referrerpolicy="no-referrer" {}
        }
    }
}

pub fn navbar_burger() -> Markup {
    html! {
        a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbar" {
            span aria-hidden="true" {}
            span aria-hidden="true" {}
            span aria-hidden="true" {}
        }
    }
}

pub fn home_navbar_item() -> Markup {
    html! {
        a.navbar-item href="/" {
            img src="/home.png" width="28" height="28" alt="Main page";
        }
    }
}

pub fn navbar_brand() -> Markup {
    html! {
        div.navbar-brand { (home_navbar_item()) (navbar_burger()) }
    }
}

pub fn profile_navbar(User { account_id, nickname, .. }: &User) -> Markup {
    html! {
        nav.navbar.is-warning role="navigation" aria-label="main navigation" {
            div.container {
                (navbar_brand())

                #navbar.navbar-menu {
                    div.navbar-start {
                        div.navbar-item {
                            span.icon { i.fa-regular.fa-user {} }
                            span { (nickname) }
                        }

                        a.navbar-item href=(format!("/profile/{account_id}")) {
                            span.icon { i.fa-solid.fa-star-half-stroke aria-hidden="true" {} }
                            span { "Rate" }
                        }

                        a.navbar-item href=(format!("/discover")) {
                            span.icon { i.fa-solid.fa-wand-magic-sparkles aria-hidden="true" {} }
                            span { "Discover" }
                        }
                    }
                    div.navbar-end {
                        div.navbar-item {
                            div.field {
                                p.control {
                                    a.button.is-rounded.is-danger href="/sign-out" {
                                        span.icon { i.fa-solid.fa-right-from-bracket {} }
                                        span { "Sign out" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn footer() -> Markup {
    html! {
        footer.footer {
            div.container {
                div.columns {
                    div.column."is-4"."is-12-tablet" {
                        p.title."is-6" { "About" }

                        p."mt-1" {
                            span.icon-text.is-flex-wrap-nowrap {
                                span.icon { i.fa-solid.fa-home.has-text-success {} }
                                span {
                                    "Website "
                                    a href=(format!("https://github.com/eigenein/blitz-tanks/releases/tag/{}", crate_version!())) {
                                        (crate_version!())
                                    }
                                    " ©️ "
                                    a href="https://github.com/eigenein" { "eigenein" }
                                }
                            }
                        }

                        p."mt-1" {
                            span.icon-text.is-flex-wrap-nowrap {
                                span.icon { i.fa-solid.fa-arrow-up-right-from-square.has-text-danger {} }
                                span {
                                    a href="https://wotblitz.com/" { "World of Tanks Blitz" }
                                    " content ©️ "
                                    a href="https://wargaming.net" { "Wargaming.net" }
                                }
                            }
                        }

                        p."mt-1" {
                            span.icon-text.is-flex-wrap-nowrap {
                                span.icon { i.fas.fa-id-badge.has-text-info {} }
                                span {
                                    a href="https://github.com/eigenein/blitz-tanks" { "Source code" }
                                    " licensed "
                                    a href="https://opensource.org/licenses/MIT" { "MIT" }
                                }
                            }
                        }
                    }
                }
            }
        }

        script { (PreEscaped(r#"document.addEventListener("DOMContentLoaded",()=>{let e=Array.prototype.slice.call(document.querySelectorAll(".navbar-burger"),0);e.forEach(e=>{e.addEventListener("click",()=>{let t=e.dataset.target,a=document.getElementById(t);e.classList.toggle("is-active"),a.classList.toggle("is-active")})})});"#)) }
    }
}

pub fn vehicle_card_image(vehicle: Option<&Vehicle>) -> Markup {
    html! {
        div.card-image {
            figure.image {
                @let url = vehicle
                    .and_then(|d| d.images.normal_url.as_ref())
                    .map_or("https://dummyimage.com/1060x774", |url| url.as_str());
                img src=(url) loading="lazy";
            }
        }
    }
}

pub fn vehicle_card_content(vehicle: Option<&Vehicle>, stats: &VehicleStats) -> Markup {
    html! {
        div.card-content {
            div.media {
                div.media-content {
                    p.title."is-5" {
                        span.icon-text {
                            span {
                                @match vehicle {
                                    Some(vehicle) => {
                                        span.has-text-warning-dark[vehicle.is_premium] { (vehicle.name) }
                                    },
                                    None => {
                                        "#" (stats.tank_id)
                                    },
                                }
                            }
                            span.icon {
                                a
                                    title="View in Armor Inspector"
                                    href=(format!("https://armor.wotinspector.com/en/blitz/{}-/", stats.tank_id))
                                {
                                    i.fa-solid.fa-arrow-up-right-from-square {}
                                }
                            }
                        }
                    }
                    @if let LocalResult::Single(timestamp) = stats.last_battle_time() {
                        p.subtitle."is-6" {
                            span.has-text-grey { "Last played" }
                            " "
                            span.has-text-weight-medium title=(timestamp) { (HumanTime::from(timestamp)) }
                        }
                    }
                }
            }
        }
    }
}

/// Render the vehicle card's footer inner HTML.
pub fn vehicle_card_footer(account_id: u32, tank_id: u16, rating: Option<Rating>) -> Markup {
    html! {
        footer.card-footer {
            a.card-footer-item.has-background-success-light[rating == Some(Rating::Like)]
                data-hx-post=(
                    if rating != Some(Rating::Like) {
                        format!("/profile/{account_id}/vehicle/{tank_id}/like")
                    } else {
                        format!("/profile/{account_id}/vehicle/{tank_id}/unrate")
                    }
                )
                data-hx-target="closest .card-footer"
                data-hx-swap="outerHTML"
            {
                span.icon-text.has-text-success[rating == Some(Rating::Like)] {
                    span.icon { i.fa-solid.fa-thumbs-up {} }
                    span { "Like" }
                }
            }
            a.card-footer-item.has-background-danger-light[rating == Some(Rating::Dislike)]
                data-hx-post=(
                    if rating != Some(Rating::Dislike) {
                        format!("/profile/{account_id}/vehicle/{tank_id}/dislike")
                    } else {
                        format!("/profile/{account_id}/vehicle/{tank_id}/unrate")
                    }
                )
                data-hx-target="closest .card-footer"
                data-hx-swap="outerHTML"
            {
                span.icon-text.has-text-danger[rating == Some(Rating::Dislike)] {
                    span.icon { i.fa-solid.fa-thumbs-down {} }
                    span { "Dislike" }
                }
            }
        }
    }
}
