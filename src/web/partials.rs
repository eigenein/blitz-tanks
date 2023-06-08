use maud::{html, Markup, PreEscaped, DOCTYPE};

pub fn head() -> Markup {
    html! {
        head {
            (DOCTYPE)
            meta name="viewport" content="width=device-width, initial-scale=1";
            meta charset="utf-8";
            title { "Recommender for World of Tanks Blitz" }
            link rel="shortcut icon" href="favicon.ico" sizes="16x16 32x32";
            link rel="apple-touch-icon" type="image/png" href="apple-touch-icon.png" sizes="180x180";
            link rel="icon" type="image/png" sizes="192x192" href="icon-192.png";
            link rel="icon" type="image/png" sizes="512x512" href="icon-512.png";
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/bulma/0.9.4/css/bulma.min.css" integrity="sha512-HqxHUkJM0SYcbvxUw5P60SzdOTy/QVwA1JJrvaXJv4q7lmbDZCmZaqz01UPOaQveoxfYRv1tHozWGPMcuTBuvQ==" crossorigin="anonymous" referrerpolicy="no-referrer";
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/bulma-prefers-dark/0.1.0-beta.1/bulma-prefers-dark.min.css" integrity="sha512-8L9NjgWBr9opkijcN9ZZCzzl7T3hVqji0baeKdTvfq1VN119XV4RNCGGI6vAF8ygQkSK0Qew84toTqqpzmbxUw==" crossorigin="anonymous" referrerpolicy="no-referrer";
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" integrity="sha512-iecdLmaskl7CVkqkXNQ/ZH/XLlvWZOJyj7Yy7tcenmpD1ypASozpmT/E0iPtmFIB46ZmdtAc9eNBvH0H/ZpiBw==" crossorigin="anonymous" referrerpolicy="no-referrer";
            script src="https://js.sentry-cdn.com/e0851235b23f439aa285aa8f0b431547.min.js" crossorigin="anonymous" {}
            script src="https://cdnjs.cloudflare.com/ajax/libs/htmx/1.9.2/htmx.min.js" integrity="sha512-ULbUWm8wCS6zRoxK/2v51vUHGhKvK8PSiqA02tyUYlYoeQm5wB8xr8lObq5zmNGpYaZsED0NLhaiPAAm2VbhXw==" crossorigin="anonymous" referrerpolicy="no-referrer" {}
        }
    }
}

pub fn navbar() -> Markup {
    html! {
        nav.navbar role="navigation" aria-label="main navigation" {
            div.container {
                div.navbar-brand {
                    a.navbar-item href="/" {
                        img src="home.png" width="28" height="28" alt="Main page";
                    }
                    a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbar" {
                        span aria-hidden="true" {}
                        span aria-hidden="true" {}
                        span aria-hidden="true" {}
                    }
                }

                #navbar.navbar-menu {
                    div.navbar-end {}
                }
            }
        }
    }
}

pub fn footer() -> Markup {
    html! {
        script { (PreEscaped(r#"document.addEventListener("DOMContentLoaded",()=>{let e=Array.prototype.slice.call(document.querySelectorAll(".navbar-burger"),0);e.forEach(e=>{e.addEventListener("click",()=>{let t=e.dataset.target,a=document.getElementById(t);e.classList.toggle("is-active"),a.classList.toggle("is-active")})})});"#)) }
    }
}
