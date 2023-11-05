use refresh_blog::*;
use markdown::{mdast::Node, to_html_with_options, to_mdast, Constructs, Options, ParseOptions};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use shared_models::*;

// CONTRACTS

/// HTTP request body to generate a post from markdown
#[derive(Serialize, Deserialize, Debug)]
pub struct GeneratePostRequest {
    /// Markdown string to generate post from
    #[serde(rename = "markdown")]
    pub markdown_str: String,
}

/// HTTP response body to generate a post from markdown
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeneratePostResponse {
    /// The generated post
    pub generated_post: Post,
}

// ERRORS

#[derive(Error, Debug)]
pub enum PublisherError {
    /// Error when an environment variable is missing
    #[error("Missing environment variable: {0}")]
    EnvMissing(String),

    /// Error when parsing markdown is unsuccessful
    #[error("Markdown parsing error: {0}")]
    ParseMdError(String),

    /// Error when parsing meta data is unsuccessful
    #[error("Error parsing meta data error: {0}")]
    ParseMetadataError(#[from] serde_yaml::Error),

    /// Error when meta data is missing
    #[error("Missing meta data")]
    MissingMetaData,

    /// Unauthenticated error
    #[error("Unauthenticated, no credentials provided")]
    Unauthenticated,

    /// Forbidden error
    #[error("Forbidden, invalid credentials provided")]
    Forbidden,

    /// Error when the auth service returns an error
    #[error("Auth service error: {0}")]
    AuthServiceError(String),

    /// Error when the authentication check request fails
    #[error("Authentication check request failed")]
    AuthCheckRequestFailed(String),
}

impl TypedErr for PublisherError {
    fn error_type(&self) -> String {
        match self {
            Self::EnvMissing(_) => "EnvMissing".to_string(),
            Self::ParseMdError(_) => "ParseMdError".to_string(),
            Self::ParseMetadataError(_) => "ParseMetadataError".to_string(),
            Self::MissingMetaData => "MissingMetaData".to_string(),
            Self::Unauthenticated => "Unauthenticated".to_string(),
            Self::Forbidden => "Forbidden".to_string(),
            Self::AuthServiceError(_) => "AuthServiceError".to_string(),
            Self::AuthCheckRequestFailed(_) => "AuthCheckRequestFailed".to_string(),
        }
    }
}

// CONFIG

/// Configuration for publisher
#[derive(Debug, Clone)]
pub struct PublisherConfig {
    /// URL to where the assets are hosted, this is where the CSS and other static assets will be
    /// fetched from
    pub assets_url: String,

    /// URL to where the base of the site is hosted, this is for navigation between the pages of
    /// the site
    pub base_url: String,

    /// URL to where htmx requests are sent to for the site, allowing for dynamic content
    pub htmx_url: String,

    /// Address for the service listen on
    pub listen_address: String,

    /// URL for the auth service used for authentication
    pub auth_url: String,

    /// Email of the admin user who is allowed to publish posts
    pub admin_email: String,
}

fn get_env_var(name: &str) -> Result<String, PublisherError> {
    std::env::var(name).map_err(|_| PublisherError::EnvMissing(name.to_string()))
}

impl PublisherConfig {
    /// Grabs the configuration from the environment variables
    ///
    /// The following environment variables are used:
    /// - `STATIC_ASSETS_PROXIED_URL`: URL to where the assets are hosted, this is where the CSS and
    ///  other static assets will be fetched from
    ///  - `BLOG_PROXIED_URL`: URL to where the base of the site is hosted, this is for
    ///  navigation between the pages of the site
    ///  - `BLOG_HTMX_PROXIED_URL`: URL to where htmx requests are sent to for the site, allowing
    ///  for dynamic content
    ///  - `SERVER_LISTEN_ADDR`: Address for the service listen on
    ///  - `ADMIN_EMAIL`: URL for the auth service used for authentication
    pub fn from_env() -> Result<Self, PublisherError> {
        let assets_url = get_env_var("STATIC_ASSETS_PROXIED_URL")?;
        let base_url = get_env_var("BLOG_PROXIED_URL")?;
        let htmx_url = get_env_var("BLOG_HTMX_PROXIED_URL")?;
        let listen_address = get_env_var("SERVER_LISTEN_ADDR")?;
        let auth_url = get_env_var("AUTH_SERVICE_URL")?;
        let admin_email = get_env_var("ADMIN_EMAIL")?;

        let config = Self {
            assets_url,
            base_url,
            htmx_url,
            listen_address,
            auth_url,
            admin_email,
        };
        Ok(config)
    }
}

impl From<PublisherConfig> for BlogConfig {
    fn from(config: PublisherConfig) -> Self {
        Self {
            assets_url: config.assets_url,
            base_url: config.base_url,
            htmx_url: config.htmx_url,
        }
    }
}

// GENERATOR

/// Generates a post from markdown
///
/// # Arguments
/// * `md_str` - Markdown string to generate post from
/// * `config` - Configuration for the post generator
///
pub fn generate_post(md_str: &str, config: BlogConfig) -> Result<Post, PublisherError> {
    let parse_options = parse_options();

    let html = to_html_with_options(
        md_str,
        &Options {
            parse: parse_options,
            ..Default::default()
        },
    )
    .map_err(|e| PublisherError::ParseMdError(e.to_string()))?;

    let meta = extract_meta(md_str)?;
    let post_content_data = PostProps {
        meta,
        post_content_html: html,
    };

    Ok(render_post(config, post_content_data))
}

/// Extracts the meta data from a markdown string
fn extract_meta(post: &str) -> Result<PostMeta, PublisherError> {
    let yaml = extract_yaml(post).ok_or(PublisherError::MissingMetaData)?;
    let meta: PostMeta = serde_yaml::from_str(&yaml)?;
    return Ok(meta);
}

/// Extracts the first YAML node value from a markdown string
///
/// Returns `None` if no YAML node is found
fn extract_yaml(post: &str) -> Option<String> {
    let ast = to_mdast(post, &parse_options()).unwrap();

    for node in ast.children().unwrap().into_iter() {
        match node {
            Node::Yaml(yaml) => return Some(yaml.value.clone()),
            _ => continue,
        }
    }
    None
}

/// Returns the md parse options used for the generator
fn parse_options() -> ParseOptions {
    ParseOptions {
        constructs: Constructs {
            frontmatter: true,
            ..Default::default()
        },
        ..Default::default()
    }
}
