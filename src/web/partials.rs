use clap::crate_version;
use maud::{html, Markup, PreEscaped, DOCTYPE};

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

pub fn footer() -> Markup {
    html! {
        footer.footer."pb-6" {
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
