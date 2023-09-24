use hyper::{
    header::HeaderValue, http::request::Parts, http::uri::InvalidUri, Body, Client, Request,
    Response, Server, Uri,
};
use hyper_tls::HttpsConnector;
use std::net::SocketAddr;
use thiserror::Error;

/* ERRORS */

type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Error, Debug)]
pub enum ReverseProxyError {
    #[error("Missing environment variable: {0}")]
    MissingConfiguration(String),
    #[error("Invalid Uri: {0:?}")]
    InvalidUri(#[from] InvalidUri),
    #[error("Failed to send proxy request: {0:?}")]
    ProxyRequestError(#[from] hyper::Error),
}

/* CONFIG */

pub struct ReverseProxyConfig {
    pub listen_addr: String,
    pub design_system_url: String,
    pub static_assets_url: String,
    pub blog_url: String,
    pub base_url: String,
}

fn get_url_from_env(key: &str) -> Result<String, ReverseProxyError> {
    std::env::var(key).map_err(|_| ReverseProxyError::MissingConfiguration(key.to_string()))
}

impl ReverseProxyConfig {
    pub fn from_env() -> Result<Self, ReverseProxyError> {
        let listen_addr = get_url_from_env("REVERSE_PROXY_LISTEN_ADDR")?;
        let design_system_url = get_url_from_env("REVERSE_PROXY_DESIGN_SYSTEM_URL")?;
        let static_assets_url = get_url_from_env("REVERSE_PROXY_STATIC_ASSETS_URL")?;
        let blog_url = get_url_from_env("REVERSE_PROXY_BLOG_URL")?;
        let base_url = get_url_from_env("REVERSE_PROXY_BASE_URL")?;
        Ok(Self {
            listen_addr,
            design_system_url,
            static_assets_url,
            blog_url,
            base_url,
        })
    }
}

/* ROUTE */

#[derive(Debug)]
pub enum Route {
    DesignSystem(String),
    StaticAssets(String),
    Blog(String),
    NotFound,
    HealthCheck,
    Root,
}

impl From<&str> for Route {
    fn from(path: &str) -> Self {
        let design_system_slug = "/design";
        let healthcheck_slug = "/healthcheck";
        let static_assets_slug = "/static";
        let blog_slug = "/blog";

        if path == "/" || path.is_empty() {
            return Route::Root;
        }

        if matches_path(path, design_system_slug) {
            return Route::DesignSystem(strip_prefix(path, design_system_slug));
        }

        if matches_path(path, blog_slug) {
            return Route::Blog(strip_prefix(path, blog_slug));
        }

        if matches_path(path, static_assets_slug) {
            return Route::StaticAssets(strip_prefix(path, static_assets_slug));
        }

        if matches_path(path, healthcheck_slug) {
            return Route::HealthCheck;
        }

        Route::NotFound
    }
}

impl From<&Uri> for Route {
    fn from(uri: &Uri) -> Self {
        Route::from(uri.path())
    }
}

impl From<&Parts> for Route {
    fn from(parts: &Parts) -> Self {
        Route::from(&parts.uri)
    }
}

/* ROUTE_HELPERS */

fn matches_path(path: &str, route_path: &str) -> bool {
    let route_path_len = route_path.len();

    if path.len() < route_path_len {
        return false;
    }

    if path.starts_with(route_path) {
        if let Some(next_char) = path.chars().nth(route_path_len) {
            return match next_char {
                '/' => true,
                _ => false,
            };
        }
        // Perfect match
        return true;
    }
    false
}

fn strip_prefix(input: &str, prefix: &str) -> String {
    if input.starts_with(prefix) {
        return input[prefix.len()..].to_string();
    }
    input.to_string()
}

/* PROXY_ROUTE */

#[derive(Debug)]
pub enum ProxyRoute {
    DesignSystem(String),
    StaticAssets(String),
    Blog(String),
}

impl ProxyRoute {
    pub fn proxied_uri(&self, config: &ReverseProxyConfig) -> String {
        get_proxied_uri(self, config)
    }
}

/* PROXY_ROUTE_HELPERS */

fn get_proxied_uri(route: &ProxyRoute, config: &ReverseProxyConfig) -> String {
    match route {
        ProxyRoute::DesignSystem(slug) => {
            format!("{}{}", config.design_system_url, slug)
        }
        ProxyRoute::StaticAssets(slug) => {
            format!("{}{}", config.static_assets_url, slug)
        }
        ProxyRoute::Blog(slug) => {
            format!("{}{}", config.blog_url, slug)
        }
    }
}

/* NON_PROXY_ROUTE */

#[derive(Debug)]
pub enum NonProxyRoute {
    NotFound,
    HealthCheck,
    Root,
}

/* CLASSIFIED_ROUTE */

#[derive(Debug)]
pub enum ClassifiedRoute {
    Proxy(ProxyRoute),
    NonProxy(NonProxyRoute),
}

impl From<Route> for ClassifiedRoute {
    fn from(route: Route) -> Self {
        match route {
            Route::Blog(slug) => ClassifiedRoute::Proxy(ProxyRoute::Blog(slug)),
            Route::DesignSystem(slug) => ClassifiedRoute::Proxy(ProxyRoute::DesignSystem(slug)),
            Route::StaticAssets(slug) => ClassifiedRoute::Proxy(ProxyRoute::StaticAssets(slug)),
            Route::NotFound => ClassifiedRoute::NonProxy(NonProxyRoute::NotFound),
            Route::HealthCheck => ClassifiedRoute::NonProxy(NonProxyRoute::HealthCheck),
            Route::Root => ClassifiedRoute::NonProxy(NonProxyRoute::Root),
        }
    }
}

impl From<&Parts> for ClassifiedRoute {
    fn from(parts: &Parts) -> Self {
        Route::from(parts).into()
    }
}

impl From<&Uri> for ClassifiedRoute {
    fn from(uri: &Uri) -> Self {
        Route::from(uri).into()
    }
}

/* GET_PROXIED_REQUEST */

fn get_proxied_request(
    request: Request<Body>,
    proxied_uri: &str,
) -> Result<Request<Body>, ReverseProxyError> {
    let (mut parts, body) = request.into_parts();

    let query = if let Some(q) = parts.uri.query() {
        format!("?{}", q.to_string())
    } else {
        "".to_string()
    };

    let fragment = if let Some(f) = parts.uri.to_string().split('#').nth(1) {
        format!("#{}", f.to_string())
    } else {
        "".to_string()
    };

    let new_uri = format!("{}{}{}", proxied_uri, query, fragment);

    parts.uri = new_uri.parse().expect("Failed to build proxied uri");

    let host = parts.uri.host().unwrap_or("");
    parts.headers.insert(
        "host",
        HeaderValue::from_str(host).expect("Failed to build host header"),
    );
    Ok(Request::from_parts(parts, body))
}

/* HANDLE_NON_PROXY_ROUTE */

fn handle_non_proxy_route(route: NonProxyRoute, base_url: &str) -> Response<Body> {
    match route {
        NonProxyRoute::NotFound => not_found_route(),
        NonProxyRoute::HealthCheck => healthcheck_route(),
        NonProxyRoute::Root => root(base_url),
    }
}

/* HANDLE_PROXY_ROUTE_HELPERS */

fn healthcheck_route() -> Response<Body> {
    Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Body::from("OK"))
        .expect("Failed to build healthcheck response")
}

fn not_found_route() -> Response<Body> {
    Response::builder()
        .status(404)
        .header("content-type", "text/html")
        .body(Body::from("Not Found"))
        .expect("Failed to build not found response")
}

fn root(base_url: &str) -> Response<Body> {
    Response::builder()
        .status(301)
        .header("Location", &format!("{}/blog", base_url))
        .body(Body::empty())
        .expect("Failed to build root response")
}

/* SEND_REQUEST */

async fn send_request(request: Request<Body>) -> Result<Response<Body>, ReverseProxyError> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut response = client.request(request).await?;
    let headers = response.headers_mut();

    headers.remove("X-Amzn-Remapped-Content-Length");
    headers.remove("X-Amzn-Remapped-Date");
    headers.remove("X-Amzn-Requestid");
    headers.remove("X-Amzn-Trace-Id");

    Ok(response)
}

/* REVERSE_PROXY_FUNCTION */

async fn reverse_proxy(req: Request<Body>) -> Result<Response<Body>, StdError> {
    let route = ClassifiedRoute::from(req.uri());
    let config = ReverseProxyConfig::from_env()?;

    println!("Request: {} {}", req.method(), req.uri());

    match route {
        ClassifiedRoute::Proxy(proxy_route) => {
            let proxied_request = get_proxied_request(req, &proxy_route.proxied_uri(&config))?;
            println!(
                "Proxied Request: {} {}",
                proxied_request.method(),
                proxied_request.uri()
            );
            let proxied_response = send_request(proxied_request).await?;
            println!("Proxied Response: {}", proxied_response.status());
            Ok(proxied_response)
        }
        ClassifiedRoute::NonProxy(non_proxy_route) => {
            let response = handle_non_proxy_route(non_proxy_route, &config.base_url);
            println!("Response: {}", response.status());
            Ok(response)
        }
    }
}

/* BOOTSTRAP / DRIVER */

#[tokio::main]
async fn main() -> Result<(), StdError> {
    let config = ReverseProxyConfig::from_env()?;
    let addr: SocketAddr = config.listen_addr.parse()?;

    let service_fn = hyper::service::make_service_fn(|_| async {
        Ok::<_, StdError>(hyper::service::service_fn(reverse_proxy))
    });

    let server = Server::bind(&addr).serve(service_fn);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    Ok(())
}
