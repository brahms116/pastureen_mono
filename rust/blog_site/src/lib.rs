use components::{layout, LayoutProps, NavbarProps};
use maud::{html, Markup, PreEscaped};

const CSS: &'static str = include_str!("../blog.css");

const POSTS_JS: &'static str = include_str!("../posts.js");

struct IndexBodyProps {
    image_src: String,
}

fn github_svg() -> Markup {
    html! {
        svg.socials__item
            xmlns="http://www.w3.org/2000/svg"
            x="0px"
            y="0px"
            viewBox="0 0 30 30"
            {
                path
                d="M15,3C8.373,3,3,8.373,3,15c0,5.623,3.872,10.328,9.092,11.63C12.036,26.468,12,26.28,12,26.047v-2.051 c-0.487,0-1.303,0-1.508,0c-0.821,0-1.551-0.353-1.905-1.009c-0.393-0.729-0.461-1.844-1.435-2.526 c-0.289-0.227-0.069-0.486,0.264-0.451c0.615,0.174,1.125,0.596,1.605,1.222c0.478,0.627,0.703,0.769,1.596,0.769 c0.433,0,1.081-0.025,1.691-0.121c0.328-0.833,0.895-1.6,1.588-1.962c-3.996-0.411-5.903-2.399-5.903-5.098 c0-1.162,0.495-2.286,1.336-3.233C9.053,10.647,8.706,8.73,9.435,8c1.798,0,2.885,1.166,3.146,1.481C13.477,9.174,14.461,9,15.495,9 c1.036,0,2.024,0.174,2.922,0.483C18.675,9.17,19.763,8,21.565,8c0.732,0.731,0.381,2.656,0.102,3.594 c0.836,0.945,1.328,2.066,1.328,3.226c0,2.697-1.904,4.684-5.894,5.097C18.199,20.49,19,22.1,19,23.313v2.734 c0,0.104-0.023,0.179-0.035,0.268C23.641,24.676,27,20.236,27,15C27,8.373,21.627,3,15,3z" {}
        }
    }
}

fn email_svg() -> Markup {
    html! {
        svg.socials__item
            xmlns="http://www.w3.org/2000/svg"
            x="0px"
            y="0px"
            viewBox="0 0 50 50"{
                path d="M12 23.403V23.39 10.389L11.88 10.3h-.01L9.14 8.28C7.47 7.04 5.09 7.1 3.61 8.56 2.62 9.54 2 10.9 2 12.41v3.602L12 23.403zM38 23.39v.013l10-7.391V12.41c0-1.49-.6-2.85-1.58-3.83-1.46-1.457-3.765-1.628-5.424-.403L38.12 10.3 38 10.389V23.39zM14 24.868l10.406 7.692c.353.261.836.261 1.189 0L36 24.868V11.867L25 20l-11-8.133V24.868zM38 25.889V41c0 .552.448 1 1 1h6.5c1.381 0 2.5-1.119 2.5-2.5V18.497L38 25.889zM12 25.889L2 18.497V39.5C2 40.881 3.119 42 4.5 42H11c.552 0 1-.448 1-1V25.889z" {}
        }
    }
}

fn linkedin_svg() -> Markup {
    html! {
    svg.socials__item
        xmlns="http://www.w3.org/2000/svg"
        x="0px"
        y="0px"
        viewBox="0 0 30 30" {
            path
                d="M24,4H6C4.895,4,4,4.895,4,6v18c0,1.105,0.895,2,2,2h18c1.105,0,2-0.895,2-2V6C26,4.895,25.105,4,24,4z M10.954,22h-2.95 v-9.492h2.95V22z M9.449,11.151c-0.951,0-1.72-0.771-1.72-1.72c0-0.949,0.77-1.719,1.72-1.719c0.948,0,1.719,0.771,1.719,1.719 C11.168,10.38,10.397,11.151,9.449,11.151z M22.004,22h-2.948v-4.616c0-1.101-0.02-2.517-1.533-2.517 c-1.535,0-1.771,1.199-1.771,2.437V22h-2.948v-9.492h2.83v1.297h0.04c0.394-0.746,1.356-1.533,2.791-1.533 c2.987,0,3.539,1.966,3.539,4.522V22z" {}
        }
    }
}

fn index_body(props: IndexBodyProps) -> Markup {
    html! {
        .index-page {
            .index-page__landing {
                .landing {
                    img.landing_image src=(props.image_src) {}
                    h1.landing_title { "Pastureen" }
                    h4.landing_subtitle { "A David Kwong blog" }
                    .landing__cta {
                        .btn {
                            a href="" {"See posts"}
                        }
                    }
                    .landing__socials{
                        .socials {
                            a href="https://github.com/brahms116" {
                                (github_svg())
                            }
                            a href="mailto:davidkwong17@gmail.com" {
                                (email_svg())
                            }
                            a href="https://www.linkedin.com/in/david-kwong-a4323b206/" {
                                (linkedin_svg())
                            }
                        }
                    }
                }
            }
            .index-page__content {
                .content-wrapper {
                    .content {
                        .about {
                            h1.about__title { "What is Pastureen?" }
                            p.about__text {
                                "Pastureen is a mix-mash of the words \"pasture\" and \"green\".
                                It comes from Psalm 23 in the Bible.
                                As I live my life out, build my projects and technologies, I hope to do so resting peacefully amongst green pastures God, my Shepard, has provided for me.
                                Pastureen is a creative outlet for me to share my journey and ideas about technologies, faith and other various facets of my life.
                                "
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct PagesConfig {
    pub assets_url: String,
    pub base_url: String,
    pub htmx_url: String,
}

pub fn index(props: PagesConfig) -> Markup {
    let navbar_props = NavbarProps {
        logo_link: props.base_url,
        logo_src: format!("{}/logo.png", props.assets_url),
        logo_text: "Pastureen".to_string(),
        nav_items: vec![],
    };

    let body_props = IndexBodyProps {
        image_src: format!("{}/logo.png", props.assets_url),
    };

    let layout_props = LayoutProps {
        title: "Pastureen".to_string(),
        custom_css: PreEscaped(CSS.to_string()),
        navbar_props,
        body: index_body(body_props),
    };

    layout(layout_props)
}


struct PostBodyProps<'a> {
    htmx_search_url: &'a str,
}

fn posts_body(props: PostBodyProps) -> Markup {
    html! {
        .content-wrapper{
            .content{
                .main-posts{
                    h1.main-posts__title { "What has David been up to?" }
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

pub fn posts_page(props: PagesConfig) -> Markup {
    let navbar_props = NavbarProps {
        logo_link: props.base_url,
        logo_src: format!("{}/logo.png", props.assets_url),
        logo_text: "Pastureen".to_string(),
        nav_items: vec![],
    };


    let layout_props = LayoutProps {
        title: "Pastureen - Posts".to_string(),
        custom_css: PreEscaped(CSS.to_string()),
        navbar_props,
        body: posts_body(PostBodyProps {
            htmx_search_url: &format!("{}/posts/search", props.htmx_url),
        }),
    };

    layout(layout_props)
}
