use maud::{html, Markup, PreEscaped};

const CSS: &'static str = include_str!("../refresh.css");
const JS: &'static str = include_str!("../refresh.js");

fn refresh_css() -> Markup {
    html! {
        style {
            (PreEscaped(CSS))
        }
    }
}

fn refresh_js() -> Markup {
    html! {
        script {
            (PreEscaped(JS))
        }
    }
}

fn correct_alpine_directives(markup: Markup) -> Markup {
    let string = markup.into_string();
    let string = string.replace("x-on:click-", "x-on:click.");
    let string = string.replace("x-on:keydown-", "x-on:keydown.");
    let string = string.replace("ctrl-k-", "ctrl.k.");
    let string = string.replace("esc-", "esc.");
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
            href="https://fonts.googleapis.com/css2?family=PT+Mono&display=swap"
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
            src="//unpkg.com/alpinejs"
            defer
        {}
    }
}

pub struct HtmxOptions {
    pub target: Option<String>,
    pub url: Option<String>,
    pub swap: Option<String>,
    pub trigger: Option<String>,
}

pub enum Actionable {
    Link(String),
    Htmx(HtmxOptions),
}

pub enum NavbarState {
    Open { search_input: String },
    Closed,
}

pub struct GlobalSearchProps {
    pub assets_url: String,
    pub state: NavbarState,
    pub input_options: HtmxOptions,
    pub search_body: Markup,
}

pub struct LayoutProps {
    pub title: String,
    pub css_src: String,
    pub custom_css: Markup,
    pub body: Markup,
    pub global_search_props: GlobalSearchProps,
}

pub fn layout(props: LayoutProps) -> Markup {
    html! {
        html {
            head {
                title { (props.title) }
                meta
                    charset="utf-8";
                meta
                    name="viewport"
                    content="width=device-width, initial-scale=1" {}
                (htmx())
                (fonts())
                (tailwind_reset())
                (alpinejs())
                // link rel="stylesheet" href=(props.css_src) {}
                (refresh_css())
                style { (props.custom_css) }
                (refresh_js())
            }
            body {
                (global_search(props.global_search_props))
                div
                    id="navbar-open-target"
                {}
                (props.body)
            }
        }
    }
}

fn search_icon_svg() -> Markup {
    html! {
        svg
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        {
            g {
                path
                    id="Vector"
                    d="M21 21L15.803 15.803M15.803 15.803C17.2096 14.3964 17.9998 12.4887 17.9998 10.4995C17.9998 8.5103 17.2096 6.60258 15.803 5.196C14.3964 3.78942 12.4887 2.99922 10.4995 2.99922C8.51029 2.99922 6.60256 3.78942 5.19599 5.196C3.78941 6.60258 2.99921 8.5103 2.99921 10.4995C2.99921 12.4887 3.78941 14.3964 5.19599 15.803C6.60256 17.2096 8.51029 17.9998 10.4995 17.9998C12.4887 17.9998 14.3964 17.2096 15.803 15.803"
                    stroke-width="1.5"
                    stroke-linecap="round"
                    stroke-linejoin="round";
            }
        }
    }
}

fn close_icon_svg() -> Markup {
    html! {
        svg
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        {
            g {
                path
                    id="Vector"
                    d="M6 18L18 6M6 6L18 18"
                    stroke-width="1.5"
                    stroke-linecap="round"
                    stroke-linejoin="round";
            }
        }
    }
}

pub struct MenuProps {
    pub sections: Vec<MenuSectionProps>,
}

pub struct MenuItemProps {
    pub label: String,
    pub actionable: Option<Actionable>,
}

pub struct MenuSectionProps {
    pub label: String,
    pub items: Vec<MenuItemProps>,
}

pub fn menu_item(props: MenuItemProps) -> Markup {
    html! {
        @match props.actionable {
            Some(Actionable::Link(url)) =>
            .menu-item {
                a href=(url) {
                    (props.label)
                }
            },
            Some(Actionable::Htmx(options)) =>
            .menu-item
                hx-get=[options.url]
                hx-trigger=[options.trigger]
                hx-swap=[options.swap]
            {
                (props.label)
            },
            None => 
            .menu-item {
                (props.label)
            },
        }
    }
}

pub fn menu_section(props: MenuSectionProps) -> Markup {
    html! {
        .menu-section {
            .menu-section__label {
                (props.label)
            }
            .menu-section__items {
                @for item in props.items {
                    (menu_item(item))
                }
            }
        }
    }
}

pub fn menu(props: MenuProps) -> Markup {
    html! {
        .menu {
            .menu__secitons {
                @for section in props.sections {
                    (menu_section(section))
                }
            } 
        }
    }
}

pub fn global_search(props: GlobalSearchProps) -> Markup {
    let search_input = match &props.state {
        NavbarState::Open { search_input } => search_input.clone(),
        NavbarState::Closed => "".to_string(),
    };

    let is_open = match &props.state {
        NavbarState::Open { search_input: _ } => true,
        NavbarState::Closed => false,
    };

    let x_data = format!("{{ isOpen: {}, searchInput: '{}' }}", is_open, search_input);

    correct_alpine_directives(html! {
        .global-search #global-search
            x-data=(x_data)
            x-on:keydown-ctrl-k-window="if (!isOpen) { isOpen=true; focusGlobalSearch() }"
            x-on:keydown-esc-window="if (isOpen) { isOpen=false; clearGlobalSearch() }"
        {
            .global-search__navbar {
                .navbar
                    x-bind:class="isOpen ? 'navbar--open' : 'navbar--closed'"
                    x-on:click="if (!isOpen) { isOpen = true; focusGlobalSearch() }"
                {
                    img.navbar__logo.pixel-art
                        src=(format!("{}/logo.png", props.assets_url))
                        alt="Pastureen" {}
                    input.navbar__input #global-search-input
                        x-cloak
                        x-ref="searchInput"
                        x-show="isOpen"
                        x-model="searchInput"
                        x-on:keydown-enter="$el.blur()"
                        hx-get=[props.input_options.url]
                        hx-trigger=[props.input_options.trigger]
                        hx-swap=[props.input_options.swap]
                        type="text"
                        {}
                    .navbar__icon
                        x-cloak
                        x-on:click-stop="isOpen=false; clearGlobalSearch()"
                        x-show="isOpen"
                    {
                        (close_icon_svg())
                    }
                    .navbar__helptext
                        x-cloak
                        x-on:click-stop="isOpen=false; clearGlobalSearch()"
                        x-show="isOpen"
                    {
                       ("ESC")
                    }
                    .navbar__input
                        x-show="!isOpen" {}
                    .navbar__icon
                        x-show="!isOpen"
                    {
                        (search_icon_svg())
                    }
                    .navbar__helptext
                        x-show="!isOpen"
                    {
                       ("CTRL+K")
                    }
                }
            }
            .global-search__body
                x-cloak
                x-show="isOpen"
                x-transition
            {
                (props.search_body)
            }
        }
    })
}
