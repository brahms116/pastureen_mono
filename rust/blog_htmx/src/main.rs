use axum::{
    extract::{Form, Json, Query, State},
    http::{HeaderValue, StatusCode, Uri},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router, Server,
};

use blog_htmx::*;
use shared_models::*;
use std::{net::SocketAddr, sync::Arc};

use tower_http::cors::{Any, CorsLayer, AllowOrigin};

pub struct JsonErrResponse(pub StatusCode, pub Json<HttpErrResponseBody>);
impl IntoResponse for JsonErrResponse {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}

impl From<BlogHtmxError> for JsonErrResponse {
    fn from(err: BlogHtmxError) -> Self {
        let status = match err {
            BlogHtmxError::ConfigurationMissing(_) | BlogHtmxError::LibrarianError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        JsonErrResponse(status, Json(HttpErrResponseBody::from(err)))
    }
}

struct BlogHtmxState {
    config: BlogHtmxConfig,
}

fn cors_predicate(headers: &HeaderValue, blog_base_url: &str) -> bool {
    let Ok(origin) = headers.to_str() else {
        return false;
    };

    if origin == blog_base_url {
        return true;
    }

    let Ok(uri) = origin.parse::<Uri>() else {
        return false;
    };

    if uri.host() == Some("localhost") || uri.host() == Some("127.0.0.1") {
        return true;
    }

    false
}

#[tokio::main]
async fn main() {
    let config = BlogHtmxConfig::from_env().unwrap_or_else(|err| {
        eprintln!(
            "Failed to get configuration from environment variables: {}",
            err
        );
        std::process::exit(1);
    });

    let base_url = config.base_url.clone();
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(move |origin: &HeaderValue, _| {
            cors_predicate(origin, &base_url)
        }))
        .allow_methods(Any)
        .allow_headers(Any);

    let state = Arc::new(BlogHtmxState {
        config: config.clone(),
    });

    let app = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/search", post(search_links))
        .route("/links", get(get_next_page_links))
        .with_state(state)
        .layer(cors);

    let socket_addr: SocketAddr = config.listen_address.parse().unwrap_or_else(|err| {
        eprintln!(
            "Failed to parse listen address `{}`: {}",
            config.listen_address, err
        );
        std::process::exit(1);
    });
    println!("Listening on {}", socket_addr);

    Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|err| {
            eprintln!("Failed to start server: {}", err);
            std::process::exit(1);
        });
}

#[derive(serde::Deserialize)]
struct SearchLinksBody {
    search: String,
}

/// Search links endpoint
async fn search_links(
    State(state): State<Arc<BlogHtmxState>>,
    Form(body): Form<SearchLinksBody>,
) -> Result<Html<String>, JsonErrResponse> {
    Ok(Html(
        render_search_results(&body.search, None, &state.config)
            .await?
            .into_string(),
    ))
}

#[derive(serde::Deserialize)]
struct GetNextPageLinksBody {
    search: String,
    #[serde(default)]
    offset: Option<u32>,
}

/// get next page for links endpoint
async fn get_next_page_links(
    State(state): State<Arc<BlogHtmxState>>,
    Query(body): Query<GetNextPageLinksBody>,
) -> Result<Html<String>, JsonErrResponse> {
    Ok(Html(
        render_search_results(&body.search, body.offset, &state.config)
            .await?
            .into_string(),
    ))
}

/// Health check endpoint
async fn healthcheck() -> &'static str {
    "OK"
}
