use super::svgs::*;
use super::utils::*;
use super::contracts::*;
use components::*;

use maud::{html, Markup, PreEscaped};

struct IndexBodyProps {
    image_src: String,
    posts_link: String,
}

fn render_index_body(props: IndexBodyProps) -> Markup {
    html! {
        .index-page {
            .index-page__landing {
                .landing {
                    img.landing_image src=(props.image_src) {}
                    h1.landing_title { "Pastureen" }
                    h4.landing_subtitle { "A David Kwong blog" }
                    .landing__cta {
                        a href="#about" {
                            .btn {
                                "What is pastureen"
                            }
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
                            h1.about__title id="about" { "What is Pastureen?" }
                            p.about__text {
                                "Pastureen is a mix-mash of the words \"pasture\" and \"green\".
                                It comes from Psalm 23 in the Bible.
                                As I live my life out, build my projects and technologies, I hope to do so resting peacefully amongst green pastures God, my Shepherd, has provided for me.
                                Pastureen is a creative outlet for me to share my journey and ideas about technologies, faith and other various facets of my life.
                                "
                            }
                            .about__cta {
                                a href=(props.posts_link) {
                                    .btn {
                                        "See posts"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}


pub fn render_index_page(props: BlogConfig) -> String {
    let navbar_props = get_navbar_props(&props, &Page::Index);

    let body_props = IndexBodyProps {
        image_src: format!("{}/logo.png", props.assets_url),
        posts_link: format!("{}/posts.html", props.base_url),
    };

    let layout_props = LayoutProps {
        title: "Pastureen".to_string(),
        custom_css: PreEscaped(CSS.to_string()),
        navbar_props,
        css_src: get_pastureen_css_src(&props),
        body: render_index_body(body_props),
    };

    layout(layout_props).into_string()
}
