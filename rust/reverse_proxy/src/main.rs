use hyper::{Body, Request, Response, Server};
use std::net::SocketAddr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReverseProxyError {
    #[error("Missing environment variable: {0}")]
    MissingConfiguration(String),
}

pub struct ReverseProxyConfig {
    pub listen_addr: String,
}

impl ReverseProxyConfig {
    pub fn from_env() -> Result<Self, ReverseProxyError> {
        let listen_addr = std::env::var("LISTEN_ADDR")
            .map_err(|_| ReverseProxyError::MissingConfiguration("LISTEN_ADDR".to_string()))?;

        Ok(Self { listen_addr })
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

async fn reverse_proxy(_req: Request<Body>) -> Result<Response<Body>, Error> {
    Ok(Response::new(Body::from("Hello World")))
}
