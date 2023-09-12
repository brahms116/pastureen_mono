use hyper::{Body, Request, Response, Server, http::request::Parts, Uri};
use std::net::SocketAddr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReverseProxyError {
    #[error("Missing environment variable: {0}")]
    MissingConfiguration(String),
}

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

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = ReverseProxyConfig::from_env()?;
    let addr: SocketAddr = config.listen_addr.parse()?;

    let service_fn = hyper::service::make_service_fn(|_| async {
        Ok::<_, Error>(hyper::service::service_fn(reverse_proxy))
    });

    let server = Server::bind(&addr).serve(service_fn);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}

pub enum Route {
    DesignSystem(String),
    NotFound,
    HealthCheck,
}

fn matches_path(path: &str, route_path: &str) -> bool {
    let route_path_len = route_path.len();

    if path.len() < route_path_len {
        return false;
    }

    if path.starts_with(route_path) {
        if let Some(next_char) = path.chars().nth(route_path_len) {
            return match next_char {
                '/' | '?' => true,
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


pub enum ProxyRoute {
    DesignSystem(String),
}

pub enum NonProxyRoute {
    NotFound,
    HealthCheck,
}

pub enum ClassifiedRoute {
    Proxy(ProxyRoute),
    NonProxy(NonProxyRoute),
}

async fn reverse_proxy(_req: Request<Body>) -> Result<Response<Body>, Error> {
    let response = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Body::from("Hello World"))
        .unwrap();

    Ok(response)
}
