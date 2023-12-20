use super::*;

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
