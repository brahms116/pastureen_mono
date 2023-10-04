use super::contracts::*;
use super::utils::*;
use components::*;

use maud::{html, Markup, PreEscaped};

struct PostBodyProps<'a> {
    pub content_html: &'a str,
    pub title: &'a str,
    pub date: &'a str,
    pub tags: &'a [&'a str],
}

fn render_post_body(props: PostBodyProps) -> Markup {
    html! {
        .content-wrapper{
            .content{
                .post{
                    h1.post__title { (props.title) }
                    h3.post__date { (props.date) }
                    .post__tags {
                        @for tag in props.tags {
                            .tag { (tag) }
                        }
                    }
                    .post__content {
                        (PreEscaped(props.content_html))
                    }
                }
            }
        }
    }
}

pub fn render_post_page(page_config: BlogConfig, props: PostProps) -> String {
    let body_props = PostBodyProps {
        content_html: &props.post_content_html,
        title: &props.meta.title,
        date: &props.meta.date,
        tags: &props
            .meta
            .tags
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>(),
    };

    let layout_props = LayoutProps {
        title: format!("Pastureen - {}", props.meta.title),
        custom_css: PreEscaped("".to_string()),
        navbar_props: get_navbar_props(&page_config, &Page::Posts),
        css_src: get_pastureen_css_src(&page_config),
        body: render_post_body(body_props),
    };
    layout(layout_props).into_string()
}

pub fn render_post(page_config: BlogConfig, props: PostProps) -> Post {
    let meta = props.meta.clone();
    let post_html = render_post_page(page_config, props);
    Post { meta, post_html }
}

