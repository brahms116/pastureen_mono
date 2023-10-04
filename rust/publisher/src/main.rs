use axum::{
    extract::{Json, State, TypedHeader},
    headers::authorization::{Authorization, Bearer},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::post,
    Router, Server,
};
use publisher::*;
use std::{net::SocketAddr, sync::Arc};

type JsonHandlerResponse<T> = Result<Json<T>, JsonErrResponse>;
pub struct JsonErrResponse(pub StatusCode, pub Json<HttpErrResponse>);
impl IntoResponse for JsonErrResponse {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}

impl From<PublisherError> for JsonErrResponse {
    fn from(err: PublisherError) -> Self {
        let status_code = match err {
            PublisherError::EnvMissing(_)
            | PublisherError::AuthServiceError(_)
            | PublisherError::AuthCheckRequestFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PublisherError::MissingMetaData
            | PublisherError::ParseMdError(_)
            | PublisherError::ParseMetadataError(_) => StatusCode::BAD_REQUEST,
            PublisherError::Unauthenticated => StatusCode::UNAUTHORIZED,
            PublisherError::Forbidden => StatusCode::FORBIDDEN,
        };

        JsonErrResponse(
            status_code,
            Json(HttpErrResponse {
                error_type: err.error_type(),
                message: err.to_string(),
            }),
        )
    }
}

struct PublisherState {
    config: PublisherConfig,
}

#[tokio::main]
async fn main() {
    let config = PublisherConfig::from_env().unwrap_or_else(|err| {
        eprintln!(
            "Failed to get configuration from environment variables: {}",
            err
        );
        std::process::exit(1);
    });

    let state = Arc::new(PublisherState {
        config: config.clone(),
    });

    let app = Router::new()
        .route("/", post(handle))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state);

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

async fn auth_middleware<B>(
    State(state): State<Arc<PublisherState>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, JsonErrResponse> {
    let token = auth.token();
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/user", state.config.auth_url))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| PublisherError::AuthCheckRequestFailed(e.to_string()))?;

    match response.status() {
        StatusCode::OK => Ok(next.run(request).await),
        StatusCode::FORBIDDEN => Err(PublisherError::Forbidden.into()),
        _ => Err(PublisherError::AuthServiceError(
            response.text().await.unwrap_or_else(|_| {
                "Failed to get error message from auth service".to_string()
            }),
        )
        .into()),
    }

}

async fn handle(
    State(state): State<Arc<PublisherState>>,
    Json(payload): Json<GeneratePostRequest>,
) -> JsonHandlerResponse<GeneratePostResponse> {
    let config = state.config.clone();
    let generate_result = generate_post(&payload.markdown_str, config.into())?;

    Ok(Json(GeneratePostResponse {
        generated_post: generate_result,
    }))
}
