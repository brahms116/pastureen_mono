use maud::{html, Markup, PreEscaped};

const CSS: &'static str = include_str!("../refresh.css");

fn refresh_css() -> Markup {
    html! {
        style {
            (PreEscaped(CSS))
        }
    }
}

fn correct_alpine_directives(markup: Markup) -> Markup {
    let string = markup.into_string();
    let string = string.replace("x-on:click-", "x-on:click.");
    let string = string.replace("x-on:keydown-", "x-on:keydown.");
    let string = string.replace("ctrl-k-", "ctrl.k.");
    let string = string.replace("-window", ".window");
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

pub struct LoaderProps {
    pub text: String,
    pub htmx_options: HtmxOptions,
}

pub fn loader(props: LoaderProps) -> Markup {
    let derived_urls = props
        .htmx_options
        .url
        .map(|url| DerivedHtmxUrls::from(url))
        .unwrap_or_default();
    html! {
        .loader
            hx-get=[derived_urls.get]
            hx-post=[derived_urls.post]
            hx-put=[derived_urls.put]
            hx-delete=[derived_urls.delete]
            hx-trigger=[props.htmx_options.trigger]
            hx-swap=[props.htmx_options.swap]
            hx-target=[props.htmx_options.target]
        {
            (props.text)
        }
    }
}

pub enum HtmxUrl {
    Get(String),
    Post(String),
    Put(String),
    Delete(String),
}

#[derive(Default)]
pub struct DerivedHtmxUrls {
    pub get: Option<String>,
    pub post: Option<String>,
    pub put: Option<String>,
    pub delete: Option<String>,
}

impl From<HtmxUrl> for DerivedHtmxUrls {
    fn from(url: HtmxUrl) -> Self {
        match url {
            HtmxUrl::Get(url) => Self {
                get: Some(url),
                post: None,
                put: None,
                delete: None,
            },
            HtmxUrl::Post(url) => Self {
                get: None,
                post: Some(url),
                put: None,
                delete: None,
            },
            HtmxUrl::Put(url) => Self {
                get: None,
                post: None,
                put: Some(url),
                delete: None,
            },
            HtmxUrl::Delete(url) => Self {
                get: None,
                post: None,
                put: None,
                delete: Some(url),
            },
        }
    }
}

pub struct HtmxOptions {
    pub target: Option<String>,
    pub url: Option<HtmxUrl>,
    pub swap: Option<String>,
    pub trigger: Option<String>,
    pub indicator: Option<String>,
}

pub enum Actionable {
    Link(String),
    Htmx(HtmxOptions),
    Alpine(String),
}

pub enum NavbarState {
    Open { search_input: String },
    Closed,
}

pub struct GlobalSearchProps {
    pub base_url: String,
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

pub struct ListItemProps {
    pub title: String,
    pub subtitle: String,
    pub tertiary: String,
    pub actionable: Option<Actionable>,
}

pub struct ListProps {
    pub items: Vec<ListItemProps>,
}

fn htmx_list_item(options: HtmxOptions, title: &str, subtitle: &str, tertiary: &str) -> Markup {
    let derived_urls = options
        .url
        .map(|url| DerivedHtmxUrls::from(url))
        .unwrap_or_default();

    html! {
        .list-item
            hx-get=[derived_urls.get]
            hx-post=[derived_urls.post]
            hx-put=[derived_urls.put]
            hx-delete=[derived_urls.delete]
            hx-trigger=[options.trigger]
            hx-swap=[options.swap]
            hx-target=[options.target]
            hx-indicator=[options.indicator]
        {
            .list-item__title {
                (title)
            }
            .list-item__subtitle {
                (subtitle)
            }
            .list-item__tertiary {
                (tertiary)
            }
        }
    }
}

pub fn list_item(props: ListItemProps) -> Markup {
    html! {
        @match props.actionable {
            Some(Actionable::Link(url)) =>
            a href=(url) {
                .list-item {
                .list-item__title {
                    (props.title)
                }
                .list-item__subtitle {
                    (props.subtitle)
                }
                .list-item__tertiary {
                    (props.tertiary)
                }
                }
            },
            Some(Actionable::Alpine(options)) =>
            .list-item
                x-on:click=(options)
            {
                .list-item__title {
                    (props.title)
                }
                .list-item__subtitle {
                    (props.subtitle)
                }
                .list-item__tertiary {
                    (props.tertiary)
                }
            },
            Some(Actionable::Htmx(options)) =>
            (htmx_list_item(options, &props.title, &props.subtitle, &props.tertiary)),
            None =>
            .list-item {
                .list-item__title {
                    (props.title)
                }
                .list-item__subtitle {
                    (props.subtitle)
                }
                .list-item__tertiary {
                    (props.tertiary)
                }
            },
        }
    }
}

pub fn list(props: ListProps) -> Markup {
    html! {
        @for item in props.items {
            (list_item(item))
        }
    }
}

fn htmx_menu_item(options: HtmxOptions, label: &str) -> Markup {
    let derived_urls = options
        .url
        .map(|url| DerivedHtmxUrls::from(url))
        .unwrap_or_default();

    html! {
        .menu-item
            hx-get=[derived_urls.get]
            hx-post=[derived_urls.post]
            hx-put=[derived_urls.put]
            hx-delete=[derived_urls.delete]
            hx-trigger=[options.trigger]
            hx-swap=[options.swap]
            hx-target=[options.target]
            hx-indicator=[options.indicator]
        {
            (label)
        }
    }
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
            Some(Actionable::Alpine(options)) =>
            .menu-item
                x-on:click-stop=(options)
            {
                (props.label)
            },
            Some(Actionable::Htmx(options)) => (htmx_menu_item(options, &props.label)),
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

    let derived_htmx_urls = props
        .input_options
        .url
        .map(|url| DerivedHtmxUrls::from(url))
        .unwrap_or_default();

    let htmx_resolved_trigger = if let Some(trigger) = props.input_options.trigger.as_ref() {
        format!("fromQuery, {}", trigger)
    } else {
        "fromQuery".to_string()
    };

    correct_alpine_directives(html! {
        .global-search #global-search
            x-data=(x_data)
            x-on:focusglobalsearch-window="
                setTimeout(() => {
                    $refs.searchInput.focus() 
                    const value = $refs.searchInput.value
                    $refs.searchInput.value = ''
                    $refs.searchInput.value = value
                }, 100)
            "
            x-on:openglobalsearch-window="
                isOpen = true
                document.body.style.overflowY='hidden'
                if ($event.detail.searchInput) {
                    searchInput = $event.detail.searchInput
                }
                if (!$event.detail.isFromQuery) {
                    $dispatch('focusglobalsearch')
                } else {
                    // We need to trigger the input manually
                    $refs.searchInput.value = searchInput
                    htmx.trigger($refs.searchInput, 'fromQuery')
                }
            "
            x-on:closeglobalsearch-window="
                isOpen = false
                $refs.searchInput.blur()
                searchInput = ''
                document.body.style.overflowY='auto'
            "
            x-init="
                $nextTick(() => {
                    const querySearch = new URLSearchParams(window.location.search).get('global-search')
                    if (querySearch) {
                        searchInput = querySearch
                        $dispatch('openglobalsearch', { isFromQuery: true })
                    }
                })
            "
            x-on:keydown-ctrl-k-window="if (!isOpen) { $event.preventDefault(); $dispatch('openglobalsearch') }"
            x-on:keydown-esc-window="if (isOpen) { $dispatch('closeglobalsearch') }"
        {
            .global-search__navbar {
                .navbar
                    x-bind:class="isOpen ? 'navbar--open' : 'navbar--closed'"
                {
                    a href=(props.base_url) {
                        img.navbar__logo.pixel-art
                            src=(format!("{}/logo.png", props.assets_url))
                            alt="Pastureen" {}
                    }
                    .navbar__body.navbar-body
                            x-on:click="if (!isOpen) { $dispatch('openglobalsearch') }"
                    {
                        input.navbar-body__input
                            x-ref="searchInput"
                            x-model="searchInput"
                            x-on:keydown-enter="$el.blur()"
                            placeholder="CLICK TO SEARCH"
                            hx-post=[derived_htmx_urls.post]
                            hx-get=[derived_htmx_urls.get]
                            hx-put=[derived_htmx_urls.put]
                            hx-delete=[derived_htmx_urls.delete]
                            hx-trigger=(htmx_resolved_trigger)
                            hx-swap=[props.input_options.swap]
                            hx-target=[props.input_options.target]
                            hx-indicator=[props.input_options.indicator]
                            name="search"
                            type="text"
                            {}
                        .navbar-body__icon
                            x-cloak
                            x-on:click-stop="$dispatch('closeglobalsearch')"
                            x-show="isOpen"
                        {
                            (close_icon_svg())
                        }
                        .navbar-body__helptext
                            x-cloak
                            x-on:click-stop="$dispatch('closeglobalsearch')"
                            x-show="isOpen"
                        {
                           ("ESC")
                        }
                        .navbar-body__icon
                            x-show="!isOpen"
                        {
                            (search_icon_svg())
                        }
                        .navbar-body__helptext
                            x-show="!isOpen"
                        {
                           ("CTRL+K")
                        }
                    }

                }
            }
            .global-search__body #global-search-body
                x-cloak
                x-show="isOpen"
                x-transition
            {
                (props.search_body)
            }
        }
    })
}
