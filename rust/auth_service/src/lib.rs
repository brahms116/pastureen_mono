use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

use sqlx::postgres::PgPool;
use sqlx::Row;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Error, Debug)]
pub enum AuthApiError {
    #[error("Missing Environment Variable {0}")]
    ConfigruationMissing(String),
    #[error("Invalid Token")]
    InvalidToken,
    #[error("Database Error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Email already exists")]
    EmailAlreadyExists,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub fname: Option<String>,
    pub lname: Option<String>,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    pub iat: u64,
    pub token_type: String,
    pub id: String
}

pub struct AuthApi {
    secret: String,
    db: PgPool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthApiConfig {
    pub secret: String,
    pub db_conn_str: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

impl AuthApi {
    fn create_access_token(&self, id: &str) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let claim = Claims {
            sub: id.to_string(),
            exp: now.as_secs() + 60 * 60,
            iat: now.as_secs(),
            token_type: "access".to_string(),
            id: Uuid::new_v4().to_string()
        };

        encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .expect("Failed to encode token")
    }

    fn create_refresh_token(&self, id: &str) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let claim = Claims {
            sub: id.to_string(),
            exp: now.as_secs() + 60 * 60 * 24 * 30,
            iat: now.as_secs(),
            token_type: "refresh".to_string(),
            id: Uuid::new_v4().to_string()
        };

        encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .expect("Failed to encode token")
    }

    /// Create an AuthApi configured from environment variables
    ///
    /// The following environment variables are used:
    /// - AUTH_SERVICE_SECRET, the secret used to sign JWTs
    /// - AUTH_SERVICE_DB_CONN_STR, the connection string of the database
    pub async fn from_env() -> Result<Self, AuthApiError> {
        let api_secret = std::env::var("AUTH_SERVICE_SECRET")
            .map_err(|_| AuthApiError::ConfigruationMissing("AUTH_SERVICE_SECRET".to_string()))?;
        let db_conn_str = std::env::var("AUTH_SERVICE_DB_CONN_STR").map_err(|_| {
            AuthApiError::ConfigruationMissing("AUTH_SERVICE_DB_CONN_STR".to_string())
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

    pub async fn get_user(&self, token: &str) -> Result<User, AuthApiError> {
        let val = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &val,
        )
        .map_err(|_| AuthApiError::InvalidToken)?;

        if token_data.claims.token_type != "access" {
            return Err(AuthApiError::InvalidToken);
        }

        let id = token_data.claims.sub;

        let query_result = sqlx::query("SELECT * FROM pastureen_user WHERE id = $1::uuid")
            .bind(id)
            .fetch_one(&self.db)
            .await?;

        let id: Uuid = query_result.try_get("id")?;
        let id = id.to_string();
        let fname: Option<String> = query_result.try_get("fname")?;
        let lname: Option<String> = query_result.try_get("lname")?;
        let email: String = query_result.try_get("email")?;

        Ok(User {
            id,
            fname,
            lname,
            email,
        })
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<TokenPair, AuthApiError> {
        let query_result = sqlx::query("SELECT * FROM pastureen_user WHERE email = $1")
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

        let id: Uuid = result.try_get("id")?;
        let id = id.to_string();

        let access_token = self.create_access_token(&id);
        let refresh_token = self.create_refresh_token(&id);

        sqlx::query("INSERT INTO refresh_token (token, user_id, root_token) VALUES ($1, $2::uuid, $3)")
            .bind(&refresh_token)
            .bind(&id)
            .bind(&refresh_token)
            .execute(&self.db)
            .await?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<TokenPair, AuthApiError> {
        let val = Validation::default();

        let token_data = decode::<Claims>(
            refresh_token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &val,
        )
        .map_err(|_| AuthApiError::InvalidToken)?;

        if token_data.claims.token_type != "refresh" {
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

        if most_recent_token.is_none() {
            return Err(AuthApiError::InvalidToken);
        }

        let most_recent_token_string: String = most_recent_token
            .unwrap()
            .try_get("token")
            .map_err(|_| AuthApiError::InvalidToken)?;

        if most_recent_token_string != refresh_token {
            sqlx::query("DELETE FROM refresh_token WHERE root_token = $1")
                .bind(&root_token)
                .execute(&self.db)
                .await?;
            return Err(AuthApiError::InvalidToken);
        }

        let user_id: Uuid = row.try_get("user_id")?;
        let user_id = user_id.to_string();
        if user_id != token_data.claims.sub {
            return Err(AuthApiError::InvalidToken);
        }

        let access_token = self.create_access_token(&user_id);
        let refresh_token = self.create_refresh_token(&user_id);

        sqlx::query("INSERT INTO refresh_token (token, user_id, root_token) VALUES ($1, $2::uuid, $3)")
            .bind(&refresh_token)
            .bind(&user_id)
            .bind(&root_token)
            .execute(&self.db)
            .await?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}
