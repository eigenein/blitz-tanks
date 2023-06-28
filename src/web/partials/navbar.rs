use maud::{html, Markup};

use crate::models::User;

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
