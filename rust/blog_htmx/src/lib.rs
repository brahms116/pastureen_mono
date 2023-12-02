use librarian_client::*;
use maud::{html, Markup};
use refresh::*;
use refresh_blog::*;
use shared_models::*;
use thiserror::Error;

const SEARCH_LIMIT: u32 = 20;

#[derive(Debug, Error)]
pub enum BlogHtmxError {
    #[error("Configuration variable {0} is missing")]
    ConfigurationMissing(String),
    #[error("Error while querying the librarian: {0}")]
    LibrarianError(#[from] ClientHttpResponseError),
}

impl TypedErr for BlogHtmxError {
    fn error_type(&self) -> String {
        match self {
            BlogHtmxError::ConfigurationMissing(_) => "ConfigurationMissing".to_string(),
            BlogHtmxError::LibrarianError(_) => "LibrarianError".to_string(),
        }
    }
}

fn get_env_var(name: &str) -> Result<String, BlogHtmxError> {
    match std::env::var(name) {
        Ok(value) => Ok(value),
        Err(_) => Err(BlogHtmxError::ConfigurationMissing(name.to_string())),
    }
}

#[derive(Clone)]
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
            assets_url: get_env_var("STATIC_ASSETS_PROXIED_URL")?,
            base_url: get_env_var("BLOG_PROXIED_URL")?,
            htmx_url: get_env_var("BLOG_HTMX_PROXIED_URL")?,
            listen_address: get_env_var("SERVER_LISTEN_ADDR")?,
        })
    }
}

/// Parses the search query and returns the query to send to the librarian
pub fn parse_search_query(
    query_str: &str,
    offset: Option<u32>,
    available_tags: &[&str],
) -> QueryLinksRequest {
    let pagination = PaginationRequest {
        page: offset.unwrap_or(0),
        limit: SEARCH_LIMIT,
    };

    let query_parts = query_str.trim().split(" ").collect::<Vec<&str>>();
    let mut title_query = String::new();
    let mut tags = Vec::new();

    let len = query_parts.len();

    for part in query_parts {
        // see if it matches "tag:tagname"
        if part.starts_with("tag:") {
            tags.push(part[4..].to_string().to_lowercase());
        } else {
            if available_tags.contains(&(part.to_lowercase().as_str())) && len == 1 {
                tags.push(part.to_string().to_lowercase());
            } else {
                title_query.push_str(part);
                title_query.push_str(" ");
            }
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
    query_str: &str,
    offset: Option<u32>,
    librarian_url: &str,
) -> Result<QueryLinksResponse, BlogHtmxError> {
    // Skip getting tags if the query is empty
    let query_links_request_body = if query_str.is_empty() {
        parse_search_query(query_str, offset, &[])
    } else {
        let available_tags = get_tags(librarian_url).await?;
        parse_search_query(
            query_str,
            offset,
            &available_tags
                .iter()
                .map(|t| t.as_str())
                .collect::<Vec<&str>>(),
        )
    };
    let response = query_links(librarian_url, query_links_request_body).await?;
    Ok(QueryLinksResponse { links: response })
}

fn link_to_item_props(link: &Link, blog_base_url: &str) -> ListItemProps {
    let mut tag_str = String::new();
    for tag in &link.tags {
        tag_str.push_str(format!("#{}", tag).as_str());
        tag_str.push_str(" ");
    }

    let item_link = if link.url.starts_with("http") {
        link.url.clone()
    } else {
        format!("{}{}", blog_base_url, link.url)
    };

    ListItemProps {
        actionable: Some(Actionable::Link(item_link)),
        title: link.title.clone(),
        subtitle: link.date.clone(),
        tertiary: tag_str,
    }
}

pub fn render_links(
    links: &[Link],
    query: &str,
    htmx_url: &str,
    blog_base_url: &str,
    next_offset: Option<u32>,
) -> Markup {
    let item_props = links
        .iter()
        .map(|link| link_to_item_props(link, blog_base_url))
        .collect::<Vec<ListItemProps>>();
    let list_props = ListProps { items: item_props };
    html! {
        (list(list_props))
        @if let Some(offset) = next_offset {
            .loader
                hx-swap="outerHTML"
                hx-trigger="intersect once"
                hx-get=(format!("{}/links?search={}&offset={}", htmx_url, query, offset))
            {
                "Loading..."
            }
        }
    }
}

pub async fn search_and_render_links(
    query_str: &str,
    offset: Option<u32>,
    config: &BlogHtmxConfig,
) -> Result<Markup, BlogHtmxError> {
    let links = search_links(query_str, offset, &config.librarian_url).await?;
    let next_offset = if links.links.len() < SEARCH_LIMIT as usize {
        None
    } else {
        Some(offset.unwrap_or(1) + 1)
    };
    Ok(render_links(
        &links.links,
        query_str,
        &config.htmx_url,
        &config.base_url,
        next_offset,
    ))
}

pub async fn render_next_page_of_links(
    query_str: &str,
    offset: Option<u32>,
    config: &BlogHtmxConfig,
) -> Result<Markup, BlogHtmxError> {
    let links_html = search_and_render_links(query_str, offset, config).await?;
    Ok(html! {
        (links_html)
    })
}

pub async fn render_search_results(
    query_str: &str,
    offset: Option<u32>,
    config: &BlogHtmxConfig,
) -> Result<Markup, BlogHtmxError> {
    let links_html = search_and_render_links(query_str, offset, config).await?;

    if let Some(_) = offset {
        return Ok(links_html)
    }

    let results_heading = if query_str.is_empty() {
        "Recent posts"
    } else {
        "Results"
    };

    Ok(render_global_search_results_page(
        GlobalSearchResultsPageProps {
            results_heading: Some(results_heading.to_string()),
            results: Some(links_html),
            loader: Some(html! {.loader{"loading..."}}),
            ..Default::default()
        },
    ))
}
