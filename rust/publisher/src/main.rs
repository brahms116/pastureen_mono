use axum::{
    extract::{Json, State, TypedHeader},
    headers::authorization::{Authorization, Bearer},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{post, get},
    Router, Server,
};

use shared_models::*;

use auth_client::*;
use auth_models::*;
use publisher::*;
use std::{net::SocketAddr, sync::Arc};

type JsonHandlerResponse<T> = Result<Json<T>, JsonErrResponse>;
pub struct JsonErrResponse(pub StatusCode, pub Json<HttpErrResponseBody>);
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

        JsonErrResponse(status_code, Json(HttpErrResponseBody::from(err)))
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
        .route("/healthcheck", get(healthcheck))
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

async fn get_user_wrapper(endpoint: &str, token: &str) -> Result<User, PublisherError> {
    get_user(endpoint, token).await.map_err(|err| match err {
        ClientHttpResponseError::RawErr(msg) => PublisherError::AuthCheckRequestFailed(msg),
        ClientHttpResponseError::TypedServiceErr(body) => {
            PublisherError::AuthServiceError(format!("{:?}", body))
        }
    })
}

async fn auth_middleware<B>(
    State(state): State<Arc<PublisherState>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, JsonErrResponse> {
    let token = auth.token();
    let endpoint = &state.config.auth_url;
    let user = get_user_wrapper(endpoint, token).await?;
    if user.email != state.config.admin_email {
        return Err(PublisherError::Forbidden.into())
    }
    Ok(next.run(request).await)
}

async fn healthcheck() -> &'static str {
    "OK"
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
