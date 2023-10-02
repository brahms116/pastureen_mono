use actix_web::{
    error::ResponseError,
    get,
    http::StatusCode,
    post,
    web::{scope, Data, Json},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

use auth_domain::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthWebServiceError {
    #[error("Missing environment variable {0}")]
    ConfigurationError(String),
    #[error(transparent)]
    ServiceError(#[from] AuthError),
    #[error("Missing token in authorization header")]
    MissingToken,
}

impl AuthWebServiceError {
    pub fn error_type(&self) -> String {
        match self {
            Self::ConfigurationError(_) => "ConfigurationError".to_string(),
            Self::ServiceError(err) => err.error_type(),
            Self::MissingToken => "MissingToken".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthWebServiceErrResponse {
    pub error_type: String,
    pub message: String,
}

impl From<&AuthError> for AuthWebServiceErrResponse {
    fn from(err: &AuthError) -> Self {
        Self {
            error_type: err.error_type(),
            message: err.to_string(),
        }
    }
}

impl From<&AuthWebServiceError> for AuthWebServiceErrResponse {
    fn from(err: &AuthWebServiceError) -> Self {
        Self {
            error_type: err.error_type(),
            message: err.to_string(),
        }
    }
}

impl ResponseError for AuthWebServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AuthWebServiceErrResponse::from(self))
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::ConfigurationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ServiceError(err) => match err {
                AuthError::ConfigruationMissing(_) | AuthError::DatabaseError(_) => {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
                AuthError::InvalidCredentials | AuthError::EmailAlreadyExists => {
                    StatusCode::BAD_REQUEST
                }
            },
            Self::MissingToken => StatusCode::UNAUTHORIZED,
        }
    }
}

pub struct AuthWebServiceConfiguration {
    pub listen_address: String,
}

impl AuthWebServiceConfiguration {
    pub fn new(listen_address: String) -> Self {
        Self { listen_address }
    }

    pub fn from_env() -> Result<Self, AuthWebServiceError> {
        let listen_address = std::env::var("AUTH_SERVICE_LISTEN_ADDR").map_err(|_| {
            AuthWebServiceError::ConfigurationError("AUTH_SERVICE_LISTEN_ADDR".to_string())
        })?;
        Ok(Self::new(listen_address))
    }
}

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Health check ok")
}

#[derive(Serialize, Deserialize)]
pub struct GetUserResponse {
    pub user: User,
}

fn get_token_from_header(req: &HttpRequest) -> Result<String, AuthWebServiceError> {
    Ok(req
        .headers()
        .get("Authorization")
        .ok_or(AuthWebServiceError::MissingToken)?
        .to_str()
        .map_err(|_| AuthWebServiceError::MissingToken)?
        .to_string())
}

#[get("")]
async fn get_user(
    req: HttpRequest,
    api: Data<Auth>,
) -> Result<Json<GetUserResponse>, AuthWebServiceError> {
    let token = get_token_from_header(&req)?;
    let user = api.get_user(&token).await?;
    Ok(Json(GetUserResponse { user }))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPairResponse {
    pub token_pair: TokenPair,
}

#[get("")]
async fn refresh_token(
    req: HttpRequest,
    api: Data<Auth>,
) -> Result<Json<TokenPairResponse>, AuthWebServiceError> {
    let token = get_token_from_header(&req)?;
    let token_pair = api.refresh(&token).await?;
    Ok(Json(TokenPairResponse { token_pair }))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("")]
async fn login(
    req: Json<LoginRequest>,
    api: Data<Auth>,
) -> Result<Json<TokenPairResponse>, AuthWebServiceError> {
    let token_pair = api.login(&req.email, &req.password).await?;
    Ok(Json(TokenPairResponse { token_pair }))
}

type StdError = Box<dyn std::error::Error + Send + Sync>;

#[actix_web::main]
async fn main() -> Result<(), StdError> {
    let api = Auth::from_env().await?;

    HttpServer::new(move || {
        let user_resource = scope("/user").service(get_user);
        let token_resource = scope("/token").service(refresh_token).service(login);
        App::new()
            .service(health_check)
            .service(user_resource)
            .service(token_resource)
            .app_data(Data::new(api.clone()))
    })
    .bind(AuthWebServiceConfiguration::from_env()?.listen_address)?
    .run()
    .await?;

    Ok(())
}
