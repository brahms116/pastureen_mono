use maud::{html, Markup, PreEscaped};
use refresh::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

const CSS: &'static str = include_str!("../blog.css");

#[derive(Error, Debug, Clone)]
pub enum BlogError {
    #[error("Environment variable is missing : {0}")]
    EnvMissing(String),
}

fn get_env(key: &str) -> Result<String, BlogError> {
    std::env::var(key).map_err(|_| BlogError::EnvMissing(key.to_string()))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogConfig {
    pub assets_url: String,
    pub base_url: String,
    pub htmx_url: String,
}

const ENV_PREFIX: &str = "BLOG_";

impl BlogConfig {
    pub fn from_env() -> Result<Self, BlogError> {
        Ok(Self {
            assets_url: get_env(&format!("{}ASSETS_URL", ENV_PREFIX))?,
            base_url: get_env(&format!("{}BASE_URL", ENV_PREFIX))?,
            htmx_url: get_env(&format!("{}HTMX_URL", ENV_PREFIX))?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostMeta {
    pub title: String,
    pub date: String,
    pub tags: Vec<String>,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostProps {
    pub meta: PostMeta,
    pub post_content_html: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub meta: PostMeta,
    pub post_html: String,
}

pub fn render_global_search_results(_query: &str) -> Markup {
    let default_menu = MenuProps {
        sections: vec![MenuSectionProps {
            label: "Common search tags".to_string(),
            items: vec![
                MenuItemProps {
                    label: "Faith".to_string(),
                    actionable: Some(
                        Actionable::Alpine("
                            searchInput+=' tag:faith'
                            $dispatch('focusglobalsearch')
                        ".to_string())
                    ),
                },
                MenuItemProps {
                    label: "Tech".to_string(),
                    actionable: Some(
                        Actionable::Alpine("
                            searchInput+=' tag:tech'
                            $dispatch('focusglobalsearch')
                        ".to_string())
                    ),
                },
                MenuItemProps {
                    label: "Music".to_string(),
                    actionable: Some(
                        Actionable::Alpine("
                            searchInput+=' tag:music'
                            $dispatch('focusglobalsearch')
                        ".to_string())
                    ),
                },
            ],
        }],
    };

    html! {
        .layout-container {
            .layout {
                .global-search-results {
                    (menu(default_menu))
                }
            }
        }
    }
}

pub fn render_index_page(config: BlogConfig) -> String {
    let layout_props = LayoutProps {
        global_search_props: GlobalSearchProps {
            assets_url: config.assets_url,
            state: NavbarState::Closed,
            input_options: HtmxOptions {
                trigger: Some("keyup changed delay:100ms".to_string()),
                target: Some("global-search__body".to_string()),
                url: Some("/search".to_string()),
                swap: Some("innerHTML".to_string()),
            },
            search_body: render_global_search_results(""),
        },
        body: PreEscaped("".to_string()),
        css_src: "".to_string(),
        custom_css: PreEscaped(CSS.to_string()),
        title: "Pastureen".to_string(),
    };

    layout(layout_props).into_string()
}
