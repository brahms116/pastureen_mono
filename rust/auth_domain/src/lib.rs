use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

use sqlx::postgres::PgPool;
use sqlx::Row;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

/// Errors that can occur when using the AuthApi
#[derive(Error, Debug)]
pub enum AuthApiError {
    /// An Env var is missing
    #[error("Missing Environment Variable {0}")]
    ConfigruationMissing(String),

    /// An invalid token was provided, this can happen if
    /// - The token is not a valid JWT
    /// - The token is not signed with the correct secret
    /// - The token is of an incorrect type
    /// - The token is expired
    #[error("Invalid Token")]
    InvalidToken,

    /// An internal database related error
    #[error("Database Error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    /// The wrong credentials were provided for retrieving a token
    #[error("Invalid credentials")]
    InvalidCredentials,

    /// This occurs when attempting to sign up with an email that already exists
    #[error("Email already exists")]
    EmailAlreadyExists,
}

impl AuthApiError {
    pub fn error_type(&self) -> String {
        match self {
            AuthApiError::ConfigruationMissing(_) => "ConfigurationError".to_string(),
            AuthApiError::InvalidToken => "InvalidToken".to_string(),
            AuthApiError::DatabaseError(_) => "DatabaseError".to_string(),
            AuthApiError::InvalidCredentials => "InvalidCredentials".to_string(),
            AuthApiError::EmailAlreadyExists => "EmailAlreadyExists".to_string(),
        }
    }
}

/// A user representation in the Auth Api
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// The first name of the user
    pub fname: String,
    /// The last name of the user
    pub lname: String,
    /// A unique email address for the user
    pub email: String,
}

/// Type of token issued by the Auth Api
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    #[serde(rename = "ACCESS")]
    Access,
    #[serde(rename = "REFRESH")]
    Refresh,
}

/// A representation of the claims the tokens provided by the Auth Api
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// The subject of the token, this is the user email
    pub sub: String,
    /// The expiration time of the token
    pub exp: u64,
    /// The time the token was issued
    pub iat: u64,
    /// The type of the token
    pub token_type: TokenType,
    /// A unique identifier for the token, this is currently just to make tokens unique and serves no other purpose
    pub id: String,
}

impl Claims {
    /// Encode the claims with a secret into a JWT, using the JWT default settings
    pub fn encode(&self, secret: &str) -> String {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("failed to encode token")
    }

    /// Decode a JWT into a Claims struct by passing a secret, using the default JWT validation settings
    pub fn from_token(token: &str, secret: &str) -> Result<Self, AuthApiError> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AuthApiError::InvalidToken)?;
        Ok(token_data.claims)
    }
}

/// Configuration for the AuthApi
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthApiConfig {
    /// The secret used to sign JWTs
    pub secret: String,
    /// The postgres connection string
    pub db_conn_str: String,
}

/// A pair of tokens, an access token and a refresh token
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

/// The AuthApi
///
/// This is the main interface for the Auth Api
/// It is used to retrieve user information and to login
#[derive(Debug, Clone)]
pub struct AuthApi {
    secret: String,
    db: PgPool,
}

impl AuthApi {
    /// Create an AuthApi configured from environment variables
    ///
    /// The following environment variables are used:
    /// - AUTH_SECRET, the secret used to sign JWTs
    /// - AUTH_DB_CONN_STR, the connection string of the database
    pub async fn from_env() -> Result<Self, AuthApiError> {
        let api_secret = std::env::var("AUTH_SECRET")
            .map_err(|_| AuthApiError::ConfigruationMissing("AUTH_SECRET".to_string()))?;
        let db_conn_str = std::env::var("AUTH_DB_CONN_STR").map_err(|_| {
            AuthApiError::ConfigruationMissing("AUTH_DB_CONN_STR".to_string())
        })?;

        Self::from_config(AuthApiConfig {
            secret: api_secret,
            db_conn_str,
        })
        .await
    }

    /// Create an AuthApi from a configuration
    ///
    /// # Arguments
    /// * `config` - The configuration to use
    pub async fn from_config(config: AuthApiConfig) -> Result<Self, AuthApiError> {
        let db = PgPool::connect(&config.db_conn_str).await?;

        Ok(Self {
            secret: config.secret,
            db,
        })
    }

    /// Retreives user information from a token
    ///
    /// If the token is invalid, a [AuthApiError::InvalidToken] is returned. Please see
    /// [AuthApiError] for more information
    ///
    /// # Arguments
    /// * `token` - The token to retrieve user information from
    pub async fn get_user(&self, token: &str) -> Result<User, AuthApiError> {
        let token_data = Claims::from_token(token, &self.secret)?;

        if token_data.token_type != TokenType::Access {
            return Err(AuthApiError::InvalidToken);
        }

        let email = token_data.sub;

        let query_result = sqlx::query(
            "SELECT 
                fname,
                lname,
                email
            FROM pastureen_user WHERE email = $1",
        )
        .bind(email)
        .fetch_one(&self.db)
        .await?;

        let fname: String = query_result.try_get("fname")?;
        let lname: String = query_result.try_get("lname")?;
        let email: String = query_result.try_get("email")?;

        Ok(User {
            fname,
            lname,
            email,
        })
    }

    /// Login a user and return a pair of tokens
    ///
    /// If the credentials are invalid, a [AuthApiError::InvalidCredentials] is returned. Please see
    /// [AuthApiError] for more information
    ///
    /// # Arguments
    /// * `email` - The email of the user
    /// * `password` - The password of the user
    ///
    pub async fn login(&self, email: &str, password: &str) -> Result<TokenPair, AuthApiError> {
        let query_result = sqlx::query(
            "SELECT 
                email,
                password
             FROM pastureen_user WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.db)
        .await?;

        let result = query_result.ok_or(AuthApiError::InvalidCredentials)?;

        let stored_password: String = result.try_get("password")?;

        // I'm storing the password in plaintext for now, as this is a hobby project, allows me to
        // manage users from the database directly.
        //
        // When the time comes, I'll migrate this service to use a proper identity provider, or
        // look at security more seriously
        if stored_password != password {
            return Err(AuthApiError::InvalidCredentials);
        }

        let email: String = result.try_get("email")?;

        let access_token = self.create_access_token(&email);
        let refresh_token = self.create_refresh_token(&email);

        sqlx::query(
            "INSERT INTO refresh_token (token, user_email, root_token) VALUES ($1, $2, $3)",
        )
        .bind(&refresh_token)
        .bind(&email)
        .bind(&refresh_token)
        .execute(&self.db)
        .await?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    /// Generates a new token pair from a refresh token
    ///
    /// The refresh token is rotated, meaning that the old refresh token will no longer be valid.
    /// In an attempt to use the old refresh token, all tokens generated from it will be invalidated as well
    ///
    /// If the refresh token is invalid, a [AuthApiError::InvalidToken] is returned. Please see
    /// [AuthApiError] for more information
    ///
    /// # Arguments
    /// * `refresh_token` - The refresh token to use
    pub async fn refresh(&self, refresh_token: &str) -> Result<TokenPair, AuthApiError> {
        let val = Validation::default();

        let token_data = decode::<Claims>(
            refresh_token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &val,
        )
        .map_err(|_| AuthApiError::InvalidToken)?;

        if token_data.claims.token_type != TokenType::Refresh {
            return Err(AuthApiError::InvalidToken);
        }

        let row = sqlx::query("SELECT * FROM refresh_token WHERE token = $1")
            .bind(&refresh_token)
            .fetch_optional(&self.db)
            .await?
            .ok_or(AuthApiError::InvalidToken)?;

        let root_token: String = row.try_get("root_token")?;

        let most_recent_token = sqlx::query(
            "SELECT * FROM refresh_token WHERE root_token = $1 ORDER BY created_at DESC LIMIT 1",
        )
        .bind(&root_token)
        .fetch_optional(&self.db)
        .await?;

        let most_recent_token_string: String = most_recent_token
            .ok_or(AuthApiError::InvalidToken)?
            .try_get("token")
            .map_err(|_| AuthApiError::InvalidToken)?;

        if most_recent_token_string != refresh_token {
            sqlx::query("DELETE FROM refresh_token WHERE root_token = $1")
                .bind(&root_token)
                .execute(&self.db)
                .await?;
            return Err(AuthApiError::InvalidToken);
        }

        let user_email: String = row.try_get("user_email")?;
        if user_email != token_data.claims.sub {
            return Err(AuthApiError::InvalidToken);
        }

        let access_token = self.create_access_token(&user_email);
        let refresh_token = self.create_refresh_token(&user_email);

        sqlx::query(
            "INSERT INTO refresh_token (token, user_email, root_token) VALUES ($1, $2, $3)",
        )
        .bind(&refresh_token)
        .bind(&user_email)
        .bind(&root_token)
        .execute(&self.db)
        .await?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    fn create_access_token(&self, id: &str) -> String {
        let now = get_epoch();
        let claim = Claims {
            sub: id.to_string(),
            exp: now + 60 * 10,
            iat: now,
            token_type: TokenType::Access,
            id: Uuid::new_v4().to_string(),
        };
        claim.encode(&self.secret)
    }

    fn create_refresh_token(&self, id: &str) -> String {
        let now = get_epoch();
        let claim = Claims {
            sub: id.to_string(),
            exp: now + 60 * 60 * 24 * 30,
            iat: now,
            token_type: TokenType::Refresh,
            id: Uuid::new_v4().to_string(),
        };
        claim.encode(&self.secret)
    }
}

fn get_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
