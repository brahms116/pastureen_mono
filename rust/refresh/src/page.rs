use super::*;

/// Props to render a full page
pub struct PageProps {
    /// The html title of the page
    pub title: String,
    /// The url to the css file, currently not used
    pub css_src: String,
    /// Custom css to be injected into the page
    pub custom_css: Markup,
    /// The body of the page
    pub body: Markup,
    /// Props for the global search component
    pub global_search_props: GlobalSearchProps,
}

/// Renders a full page with the global search component along with styles and fonts injected
pub fn page(props: PageProps) -> Markup {
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
