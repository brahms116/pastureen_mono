use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

use sqlx::postgres::PgPool;
use sqlx::Row;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

use auth_contracts::*;

/// Errors that can occur when using Auth
#[derive(Error, Debug)]
pub enum AuthError {
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

impl AuthError {
    pub fn error_type(&self) -> String {
        match self {
            AuthError::ConfigruationMissing(_) => "ConfigurationError".to_string(),
            AuthError::InvalidToken => "InvalidToken".to_string(),
            AuthError::DatabaseError(_) => "DatabaseError".to_string(),
            AuthError::InvalidCredentials => "InvalidCredentials".to_string(),
            AuthError::EmailAlreadyExists => "EmailAlreadyExists".to_string(),
        }
    }
}

/// Encode the claims with a secret into a JWT, using the JWT default settings
pub fn encode_token(claims: &Claims, secret: &str) -> String {
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("failed to encode token")
}

/// Decode a JWT into a Claims struct by passing a secret, using the default JWT validation settings
pub fn decode_token(token: &str, secret: &str) -> Result<Claims, AuthError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AuthError::InvalidToken)?;
    Ok(token_data.claims)
}

/// Configuration for Auth
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    /// The secret used to sign JWTs
    pub secret: String,
    /// The postgres connection string
    pub db_conn_str: String,
}

/// Auth
///
/// This is the main interface for Auth
/// It is used to retrieve user information and to login
#[derive(Debug, Clone)]
pub struct Auth {
    secret: String,
    db: PgPool,
}

impl Auth {
    /// Creates Auth configured from environment variables
    ///
    /// The following environment variables are used:
    /// - AUTH_SECRET, the secret used to sign JWTs
    /// - AUTH_DB_CONN_STR, the connection string of the database
    pub async fn from_env() -> Result<Self, AuthError> {
        let api_secret = std::env::var("AUTH_SECRET")
            .map_err(|_| AuthError::ConfigruationMissing("AUTH_SECRET".to_string()))?;
        let db_conn_str = std::env::var("AUTH_DB_CONN_STR")
            .map_err(|_| AuthError::ConfigruationMissing("AUTH_DB_CONN_STR".to_string()))?;

        Self::from_config(AuthConfig {
            secret: api_secret,
            db_conn_str,
        })
        .await
    }

    /// Creates Auth from a configuration
    ///
    /// # Arguments
    /// * `config` - The configuration to use
    pub async fn from_config(config: AuthConfig) -> Result<Self, AuthError> {
        let db = PgPool::connect(&config.db_conn_str).await?;

        Ok(Self {
            secret: config.secret,
            db,
        })
    }

    /// Retreives user information from a token
    ///
    /// If the token is invalid, a [AuthError::InvalidToken] is returned. Please see
    /// [AuthError] for more information
    ///
    /// # Arguments
    /// * `token` - The token to retrieve user information from
    pub async fn get_user(&self, token: &str) -> Result<User, AuthError> {
        let token_data = decode_token(token, &self.secret)?;

        if token_data.token_type != TokenType::Access {
            return Err(AuthError::InvalidToken);
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
    /// If the credentials are invalid, a [AuthError::InvalidCredentials] is returned. Please see
    /// [AuthError] for more information
    ///
    /// # Arguments
    /// * `email` - The email of the user
    /// * `password` - The password of the user
    ///
    pub async fn login(&self, email: &str, password: &str) -> Result<TokenPair, AuthError> {
        let query_result = sqlx::query(
            "SELECT 
                email,
                password
             FROM pastureen_user WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.db)
        .await?;

        let result = query_result.ok_or(AuthError::InvalidCredentials)?;

        let stored_password: String = result.try_get("password")?;

        // I'm storing the password in plaintext for now, as this is a hobby project, allows me to
        // manage users from the database directly.
        //
        // When the time comes, I'll migrate this service to use a proper identity provider, or
        // look at security more seriously
        if stored_password != password {
            return Err(AuthError::InvalidCredentials);
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
    /// If the refresh token is invalid, a [AuthError::InvalidToken] is returned. Please see
    /// [AuthError] for more information
    ///
    /// # Arguments
    /// * `refresh_token` - The refresh token to use
    pub async fn refresh(&self, refresh_token: &str) -> Result<TokenPair, AuthError> {
        let val = Validation::default();

        let token_data = decode::<Claims>(
            refresh_token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &val,
        )
        .map_err(|_| AuthError::InvalidToken)?;

        if token_data.claims.token_type != TokenType::Refresh {
            return Err(AuthError::InvalidToken);
        }

        let row = sqlx::query("SELECT * FROM refresh_token WHERE token = $1")
            .bind(&refresh_token)
            .fetch_optional(&self.db)
            .await?
            .ok_or(AuthError::InvalidToken)?;

        let root_token: String = row.try_get("root_token")?;

        let most_recent_token = sqlx::query(
            "SELECT * FROM refresh_token WHERE root_token = $1 ORDER BY created_at DESC LIMIT 1",
        )
        .bind(&root_token)
        .fetch_optional(&self.db)
        .await?;

        let most_recent_token_string: String = most_recent_token
            .ok_or(AuthError::InvalidToken)?
            .try_get("token")
            .map_err(|_| AuthError::InvalidToken)?;

        if most_recent_token_string != refresh_token {
            sqlx::query("DELETE FROM refresh_token WHERE root_token = $1")
                .bind(&root_token)
                .execute(&self.db)
                .await?;
            return Err(AuthError::InvalidToken);
        }

        let user_email: String = row.try_get("user_email")?;
        if user_email != token_data.claims.sub {
            return Err(AuthError::InvalidToken);
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
        encode_token(&claim, &self.secret)
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
        encode_token(&claim, &self.secret)
    }
}

fn get_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
