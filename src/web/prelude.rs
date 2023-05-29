use maud::DOCTYPE;
pub use maud::{html, Markup, PreEscaped};

pub fn head() -> Markup {
    html! {
        head {
            (DOCTYPE)
            meta name="viewport" content="width=device-width, initial-scale=1";
            meta charset="utf-8";
            title { "Recommender for World of Tanks Blitz" }
            link rel="icon" href="favicon.ico" sizes="any";
            link rel="apple-touch-icon" href="apple-touch-icon.png" sizes="180x180" type="image/png";
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/bulma/0.9.4/css/bulma.min.css" integrity="sha512-HqxHUkJM0SYcbvxUw5P60SzdOTy/QVwA1JJrvaXJv4q7lmbDZCmZaqz01UPOaQveoxfYRv1tHozWGPMcuTBuvQ==" crossorigin="anonymous" referrerpolicy="no-referrer";
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/bulma-prefers-dark/0.1.0-beta.1/bulma-prefers-dark.min.css" integrity="sha512-8L9NjgWBr9opkijcN9ZZCzzl7T3hVqji0baeKdTvfq1VN119XV4RNCGGI6vAF8ygQkSK0Qew84toTqqpzmbxUw==" crossorigin="anonymous" referrerpolicy="no-referrer";
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" integrity="sha512-iecdLmaskl7CVkqkXNQ/ZH/XLlvWZOJyj7Yy7tcenmpD1ypASozpmT/E0iPtmFIB46ZmdtAc9eNBvH0H/ZpiBw==" crossorigin="anonymous" referrerpolicy="no-referrer";
            script src="https://cdnjs.cloudflare.com/ajax/libs/htmx/1.9.2/htmx.min.js" integrity="sha512-ULbUWm8wCS6zRoxK/2v51vUHGhKvK8PSiqA02tyUYlYoeQm5wB8xr8lObq5zmNGpYaZsED0NLhaiPAAm2VbhXw==" crossorigin="anonymous" referrerpolicy="no-referrer" {}
        }
    }
}

pub fn navbar() -> Markup {
    html! {
        nav.navbar role="navigation" aria-label="main navigation" {
            div.container {
                div.navbar-menu.is-active {
                    div.navbar-end {
                        div.navbar-item {
                            a.button.is-link.is-rounded {
                                span.icon { i.fa-solid.fa-right-to-bracket {} }
                                strong { "Sign in" }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn footer() -> Markup {
    html! {}
}
