use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router, Server,
};
use post_generator::*;

type JsonHandlerResponse<T> = Result<Json<T>, JsonErrResponse>;
pub struct JsonErrResponse(pub StatusCode, pub Json<HttpErrResponse>);
impl IntoResponse for JsonErrResponse {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}

impl From<GeneratorError> for JsonErrResponse {
    fn from(err: GeneratorError) -> Self {
        let status_code = match err {
            GeneratorError::EnvMissing(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GeneratorError::MissingMetaData
            | GeneratorError::ParseMdError(_)
            | GeneratorError::ParseMetadataError(_) => StatusCode::BAD_REQUEST,
            GeneratorError::Unauthenticated => StatusCode::UNAUTHORIZED,
            GeneratorError::Forbidden => StatusCode::FORBIDDEN,
        };

        JsonErrResponse(
            status_code,
            Json(HttpErrResponse {
                error: err.to_string(),
            }),
        )
    }
}

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
    println!("Listening on {}", socket_addr);

    Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|err| {
            eprintln!("Failed to start server: {}", err);
            std::process::exit(1);
        });

}

async fn auth_middleware<B>()->Result<Response, StatusCode>  {

}


async fn handle(
    State(state): State<Arc<GeneratorState>>,
    Json(payload): Json<GeneratePostRequest>,
) -> JsonHandlerResponse<GeneratePostResponse> {
    let config = state.config.clone();
    let generate_result = generate_post(&payload.markdown_str, config.into())?;

    Ok(Json(GeneratePostResponse {
        generated_post: generate_result,
    }))
}
