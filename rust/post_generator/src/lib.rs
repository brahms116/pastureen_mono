use blog_site::{PagesConfig, PostMeta, RenderedPost, RenderedPostContent};
use markdown::{mdast::Node, to_html_with_options, to_mdast, Constructs, Options, ParseOptions};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
    pub generated_post: RenderedPost,
}

/// HTTP response body to an error
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HttpErrResponse {
    /// The type of error
    pub error_type: String,
    pub message: String,
}

// ERRORS

#[derive(Error, Debug)]
pub enum GeneratorError {
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

    /// Unauthorized error
    #[error("Unauthorized, invalid credentials provided")]
    Forbidden,
}

// CONFIG


/// Configuration for the post generator
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
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
}

fn get_env_var(name: &str) -> Result<String, GeneratorError> {
    std::env::var(name).map_err(|_| GeneratorError::EnvMissing(name.to_string()))
}

impl GeneratorConfig {
    /// Grabs the configuration from the environment variables
    ///
    /// The following environment variables are used:
    /// - `POST_GENERATOR_ASSETS_URL`: URL to where the assets are hosted, this is where the CSS and
    ///  other static assets will be fetched from
    ///  - `POST_GENERATOR_BASE_URL`: URL to where the base of the site is hosted, this is for
    ///  navigation between the pages of the site
    ///  - `POST_GENERATOR_HTMX_URL`: URL to where htmx requests are sent to for the site, allowing
    ///  for dynamic content
    ///  - `POST_GENERATOR_LISTEN_ADDRESS`: Address for the service listen on
    pub fn from_env() -> Result<Self, GeneratorError> {
        let assets_url = get_env_var("POST_GENERATOR_ASSETS_URL")?;
        let base_url = get_env_var("POST_GENERATOR_BASE_URL")?;
        let htmx_url = get_env_var("POST_GENERATOR_HTMX_URL")?;
        let listen_address = get_env_var("POST_GENERATOR_LISTEN_ADDRESS")?;

        let config = Self {
            assets_url,
            base_url,
            htmx_url,
            listen_address,
        };
        Ok(config)
    }
}

impl From<GeneratorConfig> for PagesConfig {
    fn from(config: GeneratorConfig) -> Self {
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
pub fn generate_post(md_str: &str, config: PagesConfig) -> Result<RenderedPost, GeneratorError> {
    let parse_options = parse_options();

    let html = to_html_with_options(
        md_str,
        &Options {
            parse: parse_options,
            ..Default::default()
        },
    )
    .map_err(|e| GeneratorError::ParseMdError(e.to_string()))?;

    let meta = extract_meta(md_str)?;
    let post_content_data = RenderedPostContent {
        meta,
        post_content_html: html,
    };

    Ok(post_content_data.render_post_page(config))
}

/// Extracts the meta data from a markdown string
fn extract_meta(post: &str) -> Result<PostMeta, GeneratorError> {
    let yaml = extract_yaml(post).ok_or(GeneratorError::MissingMetaData)?;
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
