use super::*;

pub struct MenuItemProps {
    pub label: String,
    pub actionable: Option<Actionable>,
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
