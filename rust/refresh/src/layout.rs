use super::*;

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
