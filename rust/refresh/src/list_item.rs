use super::*;

pub struct ListItemProps {
    pub title: String,
    pub subtitle: String,
    pub tertiary: String,
    pub actionable: Option<Actionable>,
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
