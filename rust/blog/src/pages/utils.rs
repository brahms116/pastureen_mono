// UTILS
use super::contracts::*;
use components::*;

pub const CSS: &'static str = include_str!("../../blog.css");

pub const POSTS_JS: &'static str = include_str!("../../posts.js");

#[derive(Clone, PartialEq)]
pub enum Page {
    Index,
    Posts,
}

pub fn get_pastureen_css_src(config: &BlogConfig) -> String {
    format!("{}/styles.css", config.assets_url)
}

pub fn get_navbar_props(config: &BlogConfig, page: &Page) -> NavbarProps {
    let base_url = &config.base_url;

    let index_link = base_url.to_string();
    let posts_link = format!("{}/posts.html", base_url);

    let nav_items: Vec<NavItemProps> = vec![
        NavItemProps {
            link: index_link.clone(),
            text: "Home".to_string(),
            is_active: page == &Page::Index,
        },
        NavItemProps {
            link: posts_link,
            text: "Posts".to_string(),
            is_active: page == &Page::Posts,
        },
    ];

    NavbarProps {
        nav_items,
        logo_link: index_link,
        logo_text: "Pastureen".to_string(),
        logo_src: format!("{}/logo.png", config.assets_url),
    }
}
