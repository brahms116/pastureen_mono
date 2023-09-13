use hyper::{
    http::request::Parts, http::uri::InvalidUri, Body, Client, Request, Response, Server, Uri,
};
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
}

impl ReverseProxyConfig {
    pub fn from_env() -> Result<Self, ReverseProxyError> {
        let listen_addr = std::env::var("REVERSE_PROXY_LISTEN_ADDR").map_err(|_| {
            ReverseProxyError::MissingConfiguration("REVERSE_PROXY_LISTEN_ADDR".to_string())
        })?;
        let design_system_url = std::env::var("REVERSE_PROXY_DESIGN_SYSTEM_URL").map_err(|_| {
            ReverseProxyError::MissingConfiguration("REVERSE_PROXY_DESIGN_SYSTEM_URL".to_string())
        })?;
        Ok(Self {
            listen_addr,
            design_system_url,
        })
    }
}


/* ROUTE */

pub enum Route {
    DesignSystem(String),
    NotFound,
    HealthCheck,
}

impl From<&str> for Route {
    fn from(path: &str) -> Self {
        let design_system_slug = "/design";
        let healthcheck_slug = "/healthcheck";

        if matches_path(path, design_system_slug) {
            return Route::DesignSystem(strip_prefix(path, design_system_slug));
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

pub enum ProxyRoute {
    DesignSystem(String),
}

impl ProxyRoute {
    pub fn proxied_path_and_query(&self) -> String {
        get_proxied_path_and_query(self)
    }
}

/* PROXY_ROUTE_HELPERS */

fn get_proxied_path_and_query(route: &ProxyRoute) -> String {
    match route {
        ProxyRoute::DesignSystem(slug) => format!("{}{}", "/design", slug),
    }
}


/* NON_PROXY_ROUTE */

pub enum NonProxyRoute {
    NotFound,
    HealthCheck,
}


/* CLASSIFIED_ROUTE */

pub enum ClassifiedRoute {
    Proxy(ProxyRoute),
    NonProxy(NonProxyRoute),
}

impl From<Route> for ClassifiedRoute {
    fn from(route: Route) -> Self {
        match route {
            Route::DesignSystem(slug) => ClassifiedRoute::Proxy(ProxyRoute::DesignSystem(slug)),
            Route::NotFound => ClassifiedRoute::NonProxy(NonProxyRoute::NotFound),
            Route::HealthCheck => ClassifiedRoute::NonProxy(NonProxyRoute::HealthCheck),
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
    proxied_path_and_query: &str,
) -> Result<Request<Body>, ReverseProxyError> {
    let (mut parts, body) = request.into_parts();
    let query = parts.uri.query().unwrap_or("");
    let proxied_uri = format!("{}?{}", proxied_path_and_query, query);

    let mut uri_parts = parts.uri.into_parts();
    uri_parts.path_and_query = Some(proxied_uri.parse()?);
    parts.uri = Uri::from_parts(uri_parts).expect("Failed to build proxied uri");
    Ok(Request::from_parts(parts, body))
}

/* HANDLE_NON_PROXY_ROUTE */

fn handle_non_proxy_route(route: NonProxyRoute) -> Response<Body> {
    match route {
        NonProxyRoute::NotFound => not_found_route(),
        NonProxyRoute::HealthCheck => healthcheck_route(),
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

/* SEND_REQUEST */

async fn send_request(request: Request<Body>) -> Result<Response<Body>, ReverseProxyError> {
    let client = Client::new();
    let response = client.request(request).await?;
    Ok(response)
}


/* REVERSE_PROXY_FUNCTION */

async fn reverse_proxy(req: Request<Body>) -> Result<Response<Body>, StdError> {
    let route = ClassifiedRoute::from(req.uri());

    match route {
        ClassifiedRoute::Proxy(proxy_route) => {
            let proxied_request = get_proxied_request(req, &get_proxied_path_and_query(&proxy_route))?;
            let proxied_response = send_request(proxied_request).await?;
            Ok(proxied_response)
        }
        ClassifiedRoute::NonProxy(non_proxy_route) => {
            let response = handle_non_proxy_route(non_proxy_route);
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
