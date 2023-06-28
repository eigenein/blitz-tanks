use clap::crate_version;
use maud::{html, Markup, PreEscaped};

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
