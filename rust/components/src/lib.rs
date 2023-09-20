use maud::{html, Markup, PreEscaped};

const PASTUREEN_CSS: &str = include_str!("../../../styles/styles.css");

fn pastureen_css() -> Markup {
    PreEscaped(PASTUREEN_CSS.to_string())
}

fn correct_alpine_directives(markup: Markup) -> Markup {
    let string = markup.into_string();
    let string = string.replace("x-on:click-outside", "x-on:click.outside");
    PreEscaped(string)
}

fn htmx() -> Markup {
    html! {
        script
            src="https://unpkg.com/htmx.org@1.9.5"
            integrity="sha384-xcuj3WpfgjlKF+FXhSQFQ0ZNr39ln+hwjN3npfM9VBnUskLolQAcN80McRIVOPuO"
            crossorigin="anonymous" {}
    }
}

fn fonts() -> Markup {
    html! {
        link
            rel="preconnect"
            href="https://fonts.gstatic.com"
            crossorigin {}
        link
            rel="preconnect"
            href="https://fonts.googleapis.com" {}
        link
            href="https://fonts.googleapis.com/css2?family=Poppins:wght@400;500;600;700;800;900&display=swap"
            rel="stylesheet" {}
    }
}

fn tailwind_reset() -> Markup {
    html! {
        link
            rel="stylesheet"
            href="https://unpkg.com/tailwindcss@3.3.3/src/css/preflight.css" {}
    }
}

fn alpinejs() -> Markup {
    html! {
        script
            src="//unpkg.com/alpinejs" {}
    }
}

pub struct LayoutProps<'a> {
    pub title: &'a str,
    pub navbar_props: NavbarProps<'a>,
}

pub fn layout(props: LayoutProps) -> Markup {
    html! {
        html {
            head {
                title { "Pastureen" }
                meta
                    charset="utf-8";
                meta
                    name="viewport"
                    content="width=device-width, initial-scale=1" {}
                (htmx())
                (fonts())
                (tailwind_reset())
                (alpinejs())
                style { (pastureen_css()) }
            }
        }
        body {
            (navbar(props.navbar_props))
            "HELLO WORLD!"
        }
    }
}

pub struct NavItemProps<'a> {
    pub link: &'a str,
    pub text: &'a str,
    pub is_active: bool,
}

pub struct NavbarProps<'a> {
    pub logo_link: &'a str,
    pub logo_src: &'a str,
    pub logo_text: &'a str,
    pub nav_items: &'a [NavItemProps<'a>],
}

fn open_menu_svg() -> Markup {
    html! {
        svg.pt-navbar-mobile-menu-button__icon
            x-show="!open"
            xmlns="http://www.w3.org/2000/svg"
            fill="none" viewBox="0 0 24 24"
            stroke-width="1.5" {
                path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M3.75 5.25h16.5m-16.5 4.5h16.5m-16.5 4.5h16.5m-16.5 4.5h16.5" {}
        }
    }
}

fn close_menu_svg() -> Markup {
    html! {
        svg.pt-navbar-mobile-menu-button__icon
            x-show="open"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5" {
                path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M6 18L18 6M6 6l12 12" {}
        }
    }
}

pub fn navbar(props: NavbarProps) -> Markup {
    html! {
        div
            x-data="{open: false}" {
                div.pt-navbar {
                    a.pt-navbar-logo
                        href=(props.logo_link) {
                            img.pt-navbar-logo__logo
                                src=(props.logo_src)
                                alt="logo" {}
                            h3.pt-navbar-logo__text { (props.logo_text) }
                    }
                    .pt-navbar-mobile-menu-button
                        x-on:click = "open = !open; document.body.style.overflowY = 'hidden'" {
                            (open_menu_svg())
                    }
                    div.pt-navbar-menu {
                        @for item in props.nav_items {
                            a.pt-navbar-menu__item.pt-navbar-menu__item--active[item.is_active]
                                    href=(item.link) {
                                        h5 { (item.text) }
                            }
                        }
                    }
                }
                div.mobile-menu
                    x-bind:style="open ? 'transform: translateX(-100%);' : 'transform: translateX(0%);'" {
                        div.pt-navbar {
                            a.pt-navbar-logo
                                href=(props.logo_link) {
                                    img.pt-navbar-logo__logo
                                        src=(props.logo_src)
                                        alt="logo" {}
                                    h3.pt-navbar-logo__text { (props.logo_text) }
                            }
                            .pt-navbar-mobile-menu-button
                                x-on:click = "open = !open; document.body.style.overflowY = 'auto'" {
                                    (close_menu_svg())
                            }
                        }
                        div.content-wrapper {
                            div.content {
                                div.mobile-menu__nav {
                                    div.mobile-menu-nav {
                                        h2.mobile-menu-nav__title {"Menu"}
                                        div.mobile-menu-nav__list {
                                            @for (i, item) in props.nav_items.iter().enumerate() {
                                                a.mobile-menu-item.mobile-menu-item--active[item.is_active]
                                                    href=(item.link) {
                                                        h5 { (item.text) }
                                                }
                                                @if i != props.nav_items.len() - 1 {
                                                    div.mobile-menu-divider {}
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
}

#[derive(Debug, Clone)]
pub enum ActionItemProps<'a> {
    Link {
        link: &'a str,
        text: &'a str,
    },
    Htmx {
        text: &'a str,
        action_target: &'a str,
        action_indicator: &'a str,
        action_link: &'a str,
    },
}

pub fn action_item(props: ActionItemProps) -> Markup {
    match props {
        ActionItemProps::Link { link, text } => {
            html! {
                a.action-menu__item
                    href=(link) {
                       (text)
                }
            }
        }
        ActionItemProps::Htmx {
            text,
            action_target,
            action_indicator,
            action_link,
        } => {
            html! {
                .action-menu__item
                    href=(action_link)
                    hx-get=(action_target)
                    hx-swap="outerHTML"
                    hx-indicator=(action_indicator) {
                       (text)
                }
            }
        }
    }
}

fn action_menu_svg() -> Markup {
    html! {
      svg.action-menu__icon
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          x-on:click="open=!open" {
              path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  d="M12 6.75a.75.75 0 110-1.5.75.75 0 010 1.5zM12 12.75a.75.75 0 110-1.5.75.75 0 010 1.5zM12 18.75a.75.75 0 110-1.5.75.75 0 010 1.5z" {}
      }
    }
}

pub struct ActionMenuProps<'a> {
    pub items: &'a [ActionItemProps<'a>],
}

fn action_menu_raw(props: ActionMenuProps) -> Markup {
    html! {
        div.action-menu
            x-data="{open: false}" {
                (action_menu_svg())
                div.action-menu__list
                    x-show="open"
                    x-on:click-outside="open=false" {
                    @for item in props.items {
                        (action_item(item.clone()))
                    }
                }
        }
    }
}

pub fn action_menu(props: ActionMenuProps) -> Markup {
    correct_alpine_directives(action_menu_raw(props))
}

pub struct ImageProps<'a> {
    pub src: &'a str,
    pub alt: &'a str,
}

pub struct ListItemProps<'a> {
    pub image: Option<ImageProps<'a>>,
    pub title: &'a str,
    pub link: Option<&'a str>,
    pub subtitle: &'a str,
    pub action_menu: Option<ActionMenuProps<'a>>,
    pub tags: Option<&'a [&'a str]>,
}

pub fn list_item<'a>(props: ListItemProps<'a>) -> Markup {
    html! {
        .resource {
            .resource__content {
                .pt-list-item {
                    @if let Some(image) = props.image {
                        img.pt-list-item__image
                            src=(image.src)
                            alt=(image.alt) {}
                    }
                    @else {
                        div.pt-list-item__image {}
                    }
                    .pt-list-item-heading.pt-list-item-heading--link[props.link.is_some()] {
                        @if let Some(link) = props.link {
                            a href=(link) {
                                h2.pt-list-item-heading__title { (props.title) }
                                h5.pt-list-item-heading__subtitle { (props.subtitle) }
                            }
                        }
                        @else {
                            h2.pt-list-item-heading__title { (props.title) }
                            h5.pt-list-item-heading__subtitle { (props.subtitle) }
                        }
                    }
                    .pt-list-item__actions {
                        @if let Some(menu_props) = props.action_menu {
                            (action_menu(menu_props))
                        }
                    }
                    .pt-list-item__tags {
                        @if let Some(tags) = props.tags {
                            @for tag in tags {
                                span.pt-list-item__tag { (tag) }
                            }
                        }
                    }
                }
            }
            .resource__loader {
               .pt-list-item-loader {
                    h2 { "Your item is loading"}
                }
            }
        }
    }
}
