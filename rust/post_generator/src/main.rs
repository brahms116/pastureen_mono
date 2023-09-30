use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::post,
    Router, Server,
};
use post_generator::*;

struct GeneratorState {
    config: GeneratorConfig,
}

#[tokio::main]
async fn main() {
    let config = GeneratorConfig::from_env().unwrap_or_else(|err| {
        eprintln!(
            "Failed to get configuration from environment variables: {}",
            err
        );
        std::process::exit(1);
    });

    let app = Router::new()
        .route("/", post(handle))
        .with_state(Arc::new(GeneratorState {
            config: config.clone(),
        }));

    let socket_addr: SocketAddr = config.listen_address.parse().unwrap_or_else(|err| {
        eprintln!(
            "Failed to parse listen address `{}`: {}",
            config.listen_address, err
        );
        std::process::exit(1);
    });

    Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|err| {
            eprintln!("Failed to start server: {}", err);
            std::process::exit(1);
        });

    println!("Listening on {}", socket_addr);
}

type JsonHandlerResponse<T> = Result<Json<T>, (StatusCode, Json<HttpErrResponse>)>;

async fn handle(
    State(state): State<Arc<GeneratorState>>,
    Json(payload): Json<GeneratePostRequest>,
) -> JsonHandlerResponse<GeneratePostResponse> {
    let config = state.config.clone();
    let generate_result = generate_post(&payload.markdown_str, config.into());

    todo!()
}
