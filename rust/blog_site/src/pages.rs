use components::{layout, LayoutProps, NavbarProps};
use maud::{html, Markup, PreEscaped};

const CSS: &'static str = include_str!("../blog.css");

struct IndexBodyProps<'a> {
    image_src: &'a str,
}

fn index_body(props: IndexBodyProps) -> Markup {
    html! {
        .index-page {
            .index-page__landing {
                .landing {
                    img.landing_image src=(props.image_src) {}
                    h1.landing_title { "Pastureen" }
                    h4.landing_subtitle { "A David Kwong blog" }
                }
            }
        }
    }
}

pub struct IndexProps<'a> {
    pub assets_url: &'a str,
    pub base_url: &'a str,
}

pub fn index(props: IndexProps) -> Markup {
    let navbar_props = NavbarProps {
        logo_link: props.base_url,
        logo_src: &format!("{}/logo.png", props.assets_url),
        logo_text: "Pastureen",
        nav_items: &vec![],
    };

    let body_props = IndexBodyProps {
        image_src: &format!("{}/landing.jpg", props.assets_url),
    };

    let layout_props = LayoutProps {
        title: "Pastureen",
        custom_css: PreEscaped(CSS.to_string()),
        navbar_props,
        body: index_body(body_props),
    };

    layout(layout_props)
}
