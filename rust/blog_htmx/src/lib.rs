use librarian_client::*;
use maud::{html, Markup};
use refresh::*;
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
    // Skip getting tags if the query is empty
    let query_request = if query.is_empty() {
        parse_search_query(query, offset, &[])
    } else {
        let available_tags = get_tags(librarian_url).await?;
        parse_search_query(
            query,
            offset,
            &available_tags
                .iter()
                .map(|t| t.as_str())
                .collect::<Vec<&str>>(),
        )
    };
    let response = query_links(librarian_url, query_request).await?;
    Ok(QueryLinksResponse { links: response })
}

fn link_to_item_props(link: &Link) -> ListItemProps {
    let mut tag_str = String::new();
    for tag in &link.tags {
        tag_str.push_str(format!("#{}", tag).as_str());
        tag_str.push_str(" ");
    }
    ListItemProps {
        actionable: Some(Actionable::Link(link.url.clone())),
        title: link.title.clone(),
        subtitle: link.date.clone(),
        tertiary: tag_str,
    }
}

pub fn render_links(
    links: &[Link],
    query: &str,
    htmx_url: &str,
    next_offset: Option<u32>,
) -> Markup {
    let item_props = links
        .iter()
        .map(link_to_item_props)
        .collect::<Vec<ListItemProps>>();
    let list_props = ListProps { items: item_props };
    html! {
        (list(list_props))
        @if let Some(offset) = next_offset {
            .loader
                hx-trigger="revealed"
                hx-get=(format!("{}/links?search={}&offset={}", htmx_url, query, offset))
            {
                "Loading..."
            }
        }
    }
}

pub async fn render_default_results(config: &BlogHtmxConfig) -> Result<Markup, BlogHtmxError> {
    render_search_results("", None, config).await
}

pub async fn render_search_results(
    query: &str,
    offset: Option<u32>,
    config: &BlogHtmxConfig,
) -> Result<Markup, BlogHtmxError> {
    let links = search_links(query, offset, &config.librarian_url).await?;
    let next_offset = if links.links.len() < SEARCH_LIMIT as usize {
        None
    } else {
        Some(offset.unwrap_or(0) + SEARCH_LIMIT)
    };
    Ok(render_links(
        &links.links,
        query,
        &config.htmx_url,
        next_offset,
    ))
}
