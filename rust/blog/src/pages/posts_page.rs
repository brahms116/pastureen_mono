use super::svgs::*;
use super::utils::*;
use super::contracts::*;
use components::*;

use maud::{html, Markup, PreEscaped};

struct PostsBodyProps<'a> {
    htmx_search_url: &'a str,
}

fn render_posts_body(props: PostsBodyProps) -> Markup {
    html! {
        .content-wrapper{
            .content{
                .main-posts{
                    h1.main-posts__title { "Me recently... " }
                    form.main-posts__search {
                        .form-item {
                            input
                                type="text"
                                name="search"
                                id="search"
                                hx-post=(props.htmx_search_url)
                                hx-trigger="customLoad,keyup changed delay:0.5s"
                                placeholder="Search posts" {}
                        }
                     }
                 }
             }
        }
        script {
            (PreEscaped(POSTS_JS.to_string()))
        }
    }
}

pub fn render_posts_page(props: BlogConfig) -> String {
    let navbar_props = get_navbar_props(&props, &Page::Posts);
    let layout_props = LayoutProps {
        title: "Pastureen - Posts".to_string(),
        custom_css: PreEscaped(CSS.to_string()),
        navbar_props,
        css_src: get_pastureen_css_src(&props),
        body: render_posts_body(PostsBodyProps {
            htmx_search_url: &format!("{}/posts/search", props.htmx_url),
        }),
    };
    layout(layout_props).into_string()
}
