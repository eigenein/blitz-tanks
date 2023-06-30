use chrono_humanize::HumanTime;
use maud::{html, Markup, Render};

use crate::{
    models::{Rating, Vehicle, VehicleAvailability},
    prelude::*,
};

#[must_use]
pub struct VehicleCard {
    vehicle: &'static Vehicle,
    last_battle_time: Option<DateTime>,
    title_style: &'static str,
    rating: Option<(u32, Option<Rating>)>,
    predicted_rating: Option<f64>,
}

impl VehicleCard {
    pub const fn new(vehicle: &'static Vehicle) -> Self {
        Self {
            vehicle,
            last_battle_time: None,
            title_style: "is-5",
            rating: None,
            predicted_rating: None,
        }
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
        self.predicted_rating = Some(rating);
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
                        href=(format!("https://armor.wotinspector.com/en/blitz/{}-/", self.vehicle.tank_id))
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

impl Render for VehicleCard {
    fn render(&self) -> Markup {
        html! {
            div.card {
                div.card-image {
                    figure.image."is-3by2" {
                        img.has-object-fit-contain
                            src=(format!("/static/vehicle/{}?v2", self.vehicle.tank_id))
                            loading="lazy";
                    }
                }

                div.card-content {
                    div.media {
                        div.media-content {
                            p.title.(self.title_style) {
                                span.icon-text.is-flex-wrap-nowrap {
                                    span
                                        .has-text-warning-dark[self.vehicle.availability == VehicleAvailability::Premium]
                                        .has-text-info-dark[self.vehicle.availability == VehicleAvailability::Collectible]
                                    {
                                        (self.vehicle.name)
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
                            value=(predicted_rating.clamp(0.0, Rating::Like.into_f64()))
                        {
                            (predicted_rating)
                        }
                    }

                    div.field.is-horizontal {
                        div.field-body {
                            @if let Some((account_id, rating)) = self.rating {
                                (Self::vehicle_rate_buttons(account_id, self.vehicle.tank_id, rating))
                            }
                            (self.vehicle_inspector_button(self.rating.is_none()))
                        }
                    }
                }
            }
        }
    }
}
