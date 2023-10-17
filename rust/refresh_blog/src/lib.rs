use serde::{Deserialize, Serialize};
use thiserror::Error;
use refresh::*;
use maud::PreEscaped;

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


pub fn render_index_page(config: BlogConfig) -> String {
    let layout_props = LayoutProps {
        navbar_props: NavbarProps {
            htmx_url: config.htmx_url,
            assets_url: config.assets_url,
        },
        body: PreEscaped("".to_string()),
        css_src: "".to_string(),
        custom_css: PreEscaped("".to_string()),
        title: "Pastureen".to_string(),
    };

    layout(layout_props).into_string()
}
