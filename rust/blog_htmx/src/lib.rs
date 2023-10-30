use librarian_client::*;
use maud::{html, Markup};
use shared_models::*;
use thiserror::Error;

const SEARCH_LIMIT: u32 = 10;

const ENV_PREFIX: &str = "BLOG_HTMX_";

#[derive(Debug, Error)]
pub enum BlogHtmxError {
    #[error("Configuration variable {0} is missing")]
    ConfigurationMissing(String),
    #[error("Error while querying the librarian: {0}")]
    LibrarianError(#[from] ClientHttpResponseError),
}

fn get_env_var(name: &str) -> Result<String, BlogHtmxError> {
    match std::env::var(format!("{}{}", ENV_PREFIX, name)) {
        Ok(value) => Ok(value),
        Err(_) => Err(BlogHtmxError::ConfigurationMissing(name.to_string())),
    }
}

pub struct BlogHtmxConfig {
    pub librarian_url: String,
    pub assets_url: String,
    pub base_url: String,
    pub htmx_url: String,
    pub listen_address: String,
}

impl BlogHtmxConfig {
    pub fn from_env() -> Result<Self, BlogHtmxError> {
        Ok(BlogHtmxConfig {
            librarian_url: get_env_var("LIBRARIAN_URL")?,
            assets_url: get_env_var("ASSETS_URL")?,
            base_url: get_env_var("BASE_URL")?,
            htmx_url: get_env_var("HTMX_URL")?,
            listen_address: get_env_var("LISTEN_ADDRESS")?,
        })
    }
}

/// Parses the search query and returns the query to send to the librarian
pub fn parse_search_query(
    query: &str,
    offset: Option<u32>,
    available_tags: &[&str],
) -> QueryLinksRequest {
    let pagination = PaginationRequest {
        page: offset.unwrap_or(0),
        limit: SEARCH_LIMIT,
    };

    let query_parts = query.split(" ").collect::<Vec<&str>>();
    let mut title_query = String::new();
    let mut tags = Vec::new();

    for part in query_parts {
        // see if it matches "tag:tagname"
        if part.starts_with("tag:") {
            tags.push(part[4..].to_string());
        } else {
            if available_tags.contains(&part) {
                tags.push(part.to_string());
            }
            title_query.push_str(part);
            title_query.push_str(" ");
        }
    }

    QueryLinksRequest {
        pagination,
        tags,
        title_query,
        start_date: String::new(),
        end_date: String::new(),
    }
}

pub async fn search_links(
    query: &str,
    offset: Option<u32>,
    librarian_url: &str,
) -> Result<QueryLinksResponse, BlogHtmxError> {
    let available_tags = get_tags(librarian_url).await?;
    let query = parse_search_query(
        query,
        offset,
        &available_tags
            .iter()
            .map(|t| t.as_str())
            .collect::<Vec<&str>>(),
    );

    let response = query_links(librarian_url, query).await?;

    Ok(QueryLinksResponse { links: response })
}

pub fn render_links(links: &[Link], next_offset: Option<u32>) -> Markup {
    todo!()
}

pub async fn render_default_results(config: &BlogHtmxConfig) -> Result<Markup, BlogHtmxError> {
    todo!()
}

pub async fn render_search_results(
    query: &str,
    config: &BlogHtmxConfig,
) -> Result<Markup, BlogHtmxError> {
    todo!()
}

