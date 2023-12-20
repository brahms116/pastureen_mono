use maud::{html, Markup, PreEscaped};
const CSS: &'static str = include_str!("../refresh.css");

pub fn refresh_css() -> Markup {
    html! {
        style {
            (PreEscaped(CSS))
        }
    }
}

pub fn htmx() -> Markup {
    html! {
        script
            src="https://unpkg.com/htmx.org@1.9.5"
            integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO"
            crossorigin="anonymous" {}
    }
}

pub fn fonts() -> Markup {
    html! {
        link
            rel="preconnect"
            href="https://fonts.gstatic.com"
            crossorigin {}
        link
            rel="preconnect"
            href="https://fonts.googleapis.com" {}
        link
            href="https://fonts.googleapis.com/css2?family=PT+Mono&display=swap"
            rel="stylesheet" {}
    }
}

pub fn tailwind_reset() -> Markup {
    html! {
        link
            rel="stylesheet"
            href="https://unpkg.com/tailwindcss@3.3.3/src/css/preflight.css" {}
    }
}

pub fn alpinejs() -> Markup {
    html! {
        script
            src="//unpkg.com/alpinejs"
            defer
        {}
    }
}
