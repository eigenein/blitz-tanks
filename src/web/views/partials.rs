use chrono_humanize::HumanTime;
use clap::crate_version;
use maud::{html, Markup, PreEscaped, Render, DOCTYPE};

use crate::{
    models::{rating::Rating, user::User, vehicle::Vehicle},
    prelude::DateTime,
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
            meta name="htmx-config" content=r#"{"requestClass": "is-loading"}"#;
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

#[must_use]
pub struct VehicleCard<'a> {
    tank_id: u16,
    vehicle: Option<&'a Vehicle>,
    last_battle_time: Option<DateTime>,
    title_style: &'static str,
    rating: Option<(u32, Option<Rating>)>,
    predicted_rating: Option<f64>,
}

impl<'a> VehicleCard<'a> {
    pub const fn new(tank_id: u16) -> Self {
        Self {
            tank_id,
            vehicle: None,
            last_battle_time: None,
            title_style: "is-5",
            rating: None,
            predicted_rating: None,
        }
    }

    pub fn tankopedia(&mut self, vehicle: impl Into<Option<&'a Vehicle>>) -> &mut Self {
        self.vehicle = vehicle.into();
        self
    }

    pub fn last_battle_time(&mut self, last_battle_time: impl Into<Option<DateTime>>) -> &mut Self {
        self.last_battle_time = last_battle_time.into();
        self
    }

    pub fn title_style(&mut self, style: &'static str) -> &mut Self {
        self.title_style = style;
        self
    }

    pub fn rating(&mut self, account_id: u32, rating: Option<Rating>) -> &mut Self {
        self.rating = Some((account_id, rating));
        self
    }

    pub fn predicted_rating(&mut self, rating: f64) -> &mut Self {
        self.predicted_rating = Some(rating.clamp(0.0, Rating::Like.into_f64()));
        self
    }

    pub fn vehicle_rate_buttons(
        account_id: u32,
        tank_id: u16,
        current_rating: Option<Rating>,
    ) -> Markup {
        let is_liked = current_rating == Some(Rating::Like);
        let is_disliked = current_rating == Some(Rating::Dislike);

        html! {
            div.field.has-addons.is-fullwidth {
                p.control.is-expanded {
                    a.button.is-small.is-rounded.is-fullwidth.is-success[is_liked].is-selected[is_liked]
                        data-hx-post=(
                            if !is_liked {
                                format!("/profile/{account_id}/vehicle/{tank_id}/like")
                            } else {
                                format!("/profile/{account_id}/vehicle/{tank_id}/unrate")
                            }
                        )
                        data-hx-target="closest .field"
                        data-hx-swap="outerHTML"
                    {
                        span.icon.is-small { i.fa-solid.fa-thumbs-up {} }
                        span { "Like" }
                    }
                }

                p.control.is-expanded {
                    a.button.is-small.is-rounded.is-fullwidth.is-danger[is_disliked].is-selected[is_disliked]
                        data-hx-post=(
                            if !is_disliked {
                                format!("/profile/{account_id}/vehicle/{tank_id}/dislike")
                            } else {
                                format!("/profile/{account_id}/vehicle/{tank_id}/unrate")
                            }
                        )
                        data-hx-target="closest .field"
                        data-hx-swap="outerHTML"
                    {
                        span.icon.is-small { i.fa-solid.fa-thumbs-down {} }
                        span { "Dislike" }
                    }
                }
            }
        }
    }

    fn vehicle_inspector_button(&self, is_expanded: bool) -> Markup {
        html! {
            div.field.is-fullwidth {
                p.control.is-expanded {
                    a.button.is-small.is-rounded.is-fullwidth
                        title="View in Armor Inspector (external resource)"
                        href=(format!("https://armor.wotinspector.com/en/blitz/{}-/", self.tank_id))
                    {
                        span.icon.is-small { i.fa-solid.fa-arrow-up-right-from-square {} }
                        @if is_expanded {
                            span { "Inspector" }
                        }
                    }
                }
            }
        }
    }
}

impl<'a> Render for VehicleCard<'a> {
    fn render(&self) -> Markup {
        html! {
            div.card {
                div.card-image {
                    figure.image."is-3by2".has-object-fit-cover {
                        @let url = self
                            .vehicle
                            .and_then(|d| d.images.normal_url.as_ref())
                            .map_or("https://dummyimage.com/1080x720", |url| url.as_str());
                        img src=(url) loading="lazy";
                    }
                }

                div.card-content {
                    div.media {
                        div.media-content {
                            p.title.(self.title_style) {
                                span.icon-text.is-flex-wrap-nowrap {
                                    span {
                                        @match self.vehicle {
                                            Some(vehicle) => {
                                                span.has-text-warning-dark[vehicle.is_premium] { (vehicle.name) }
                                            },
                                            None => { "#" (self.tank_id) },
                                        }
                                    }
                                }
                            }

                            @if let Some(last_battle_time) = self.last_battle_time {
                                p.subtitle."is-6" {
                                    span.has-text-grey { "Last played" }
                                    " "
                                    span.has-text-weight-medium title=(last_battle_time) { (HumanTime::from(last_battle_time)) }
                                }
                            }
                        }
                    }

                    @if let Some(predicted_rating) = self.predicted_rating {
                        progress.progress.is-small
                            max=(Rating::Like.into_f64())
                            value=(predicted_rating)
                        {
                            (predicted_rating)
                        }
                    }

                    div.field.is-horizontal {
                        div.field-body {
                            @if let Some((account_id, rating)) = self.rating {
                                (Self::vehicle_rate_buttons(account_id, self.tank_id, rating))
                            }
                            (self.vehicle_inspector_button(self.rating.is_none()))
                        }
                    }
                }
            }
        }
    }
}
