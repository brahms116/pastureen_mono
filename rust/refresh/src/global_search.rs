use super::*;

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
