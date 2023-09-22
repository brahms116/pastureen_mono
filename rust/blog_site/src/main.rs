mod pages;
use pages::*;
use std::fs::{File, OpenOptions};
use std::io::Write;

#[cfg(feature = "local")]
use axum::Router;

#[cfg(feature = "local")]
use tower_http::services::ServeDir;

#[cfg(feature = "local")]
use std::net::SocketAddr;

struct BlogSiteConfig {
    pub base_url: String,
    pub assets_url: String,
}

impl BlogSiteConfig {
    fn new(base_url: String, assets_url: String) -> Self {
        Self {
            base_url,
            assets_url,
        }
    }

    fn from_env() -> Self {
        let base_url = std::env::var("BLOG_SITE_BASE_URL").expect("BLOG_SITE_BASE_URL not set");
        let assets_url =
            std::env::var("BLOG_SITE_ASSETS_URL").expect("BLOG_SITE_ASSETS_URL not set");
        Self::new(base_url, assets_url)
    }
}

fn get_file_descriptor(path: &str) -> File {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect(&format!("Could not open file {}", path))
}

fn build() {
    let config = BlogSiteConfig::from_env();
    let index_props = IndexProps {
        assets_url: &config.assets_url,
        base_url: &config.base_url,
    };

    let mut index_file = get_file_descriptor("./build/index.html");

    index_file
        .write_all(index(index_props).into_string().as_bytes())
        .expect("Could not write to index file");
}

#[cfg(not(feature = "local"))]
fn main() {
    build()
}

#[cfg(feature = "local")]
#[tokio::main]
async fn main() {
    build();

    let app = Router::new().nest_service("/", ServeDir::new("build"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8082));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
