use auth_domain::*;
use sqlx::postgres::PgPool;
use sqlx::Row;
use uuid::Uuid;
use auth_models::*;


pub struct SetupTokenPairOutput {
    pub email: String,
    pub password: String,
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn setup_token_pair(api: &Auth)-> SetupTokenPairOutput {
    let email = format!("{}@login.com", Uuid::new_v4().to_string());
    insert_user(&email, "password").await;
    let res = api.login(&email, "password").await.unwrap();

    let access_token = res.access_token;
    let refresh_token = res.refresh_token;

    SetupTokenPairOutput {
        email,
        password: "password".to_string(),
        access_token,
        refresh_token,
    }
}

pub async fn get_api() -> Auth {
    Auth::from_env().await.unwrap()
}

fn get_connection_string() -> String {
    std::env::var("AUTH_API_DB_CONN_STR").unwrap()
}

fn get_secret() -> String {
    std::env::var("AUTH_API_SECRET").unwrap()
}

pub async fn insert_user(email: &str, password: &str) -> String {
    let pool = PgPool::connect(get_connection_string().as_str())
        .await
        .unwrap();

    let result = sqlx::query("INSERT INTO pastureen_user (email, password, fname, lname) VALUES ($1, $2, $3, $4) RETURNING email")
        .bind(email)
        .bind(password)
        .bind("fname")
        .bind("lname")
        .fetch_one(&pool)
        .await
        .unwrap();

    result.try_get("email").unwrap()
}


pub async fn delete_user(email: &str) {
    let pool = PgPool::connect(get_connection_string().as_str())
        .await
        .unwrap();

    sqlx::query("DELETE FROM pastureen_user WHERE email = $1")
        .bind(email)
        .execute(&pool)
        .await
        .unwrap();
}

pub fn get_expired_access_token(email: &str) -> String {
    let claims = Claims {
        sub: email.to_string(),
        token_type: TokenType::Access,
        iat: 0,
        exp: 0,
        id: Uuid::new_v4().to_string()
    };
    encode_token(&claims, &get_secret())
}

pub fn get_expired_refresh_token(email: &str) -> String {
    let claims = Claims {
        sub: email.to_string(),
        token_type: TokenType::Refresh,
        iat: 0,
        exp: 0,
        id: Uuid::new_v4().to_string()
    };
    encode_token(&claims, &get_secret())
}

pub fn decode_token_helper(token: &str) -> Claims {
    decode_token(token, &get_secret()).unwrap()
}
