use reqwest::Client;
use reqwest_utils::*;
use serde::{Deserialize, Serialize};
use shared_models::*;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub title: String,
    pub date: String,
    pub url: String,
    pub subtitle: String,
    pub description: String,
    pub image_url: Option<String>,
    pub image_alt: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationRequest {
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryLinksRequest {
    pub pagination: PaginationRequest,
    pub tags: Vec<String>,
    pub title_query: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryLinksResponse {
    pub links: Vec<Link>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTagsResponse {
    pub tags: Vec<String>,
}

/// Queries the Librarian Api and retireves the list of tags
///
/// # Arguments
/// * `endpoint` - The endpoint of the Librarian Api
pub async fn get_tags(endpoint: &str) -> Result<Vec<String>, ClientHttpResponseError> {
    let client = Client::new();

    let res = client.get(&format!("{}/tags", endpoint)).send().await;
    handle_res::<GetTagsResponse>(res).await.map(|r| r.tags)
}

/// Searches links in the Librarian Api
///
/// # Arguements
/// * `endpoint` - The endpoint of the Librarian Api
/// * `query` - The query to search for
///
pub async fn query_links(
    endpoint: &str,
    query: QueryLinksRequest,
) -> Result<Vec<Link>, ClientHttpResponseError> {
    let client = Client::new();

    let res = client
        .post(&format!("{}/links", endpoint))
        .json(&query)
        .send()
        .await;

    handle_res::<QueryLinksResponse>(res).await.map(|r| r.links)
}
