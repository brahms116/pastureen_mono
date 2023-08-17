use auth_service::Claims;
use jsonwebtoken::{decode, DecodingKey, Validation, EncodingKey, Header, encode};
use sqlx::postgres::PgPool;
use sqlx::Row;
use uuid::Uuid;


pub struct SetupTokenPairOutput {
    pub email: String,
    pub user_id: String,
    pub password: String,
    pub access_token: String,
    pub refresh_token: String,
}

pub async fn setup_token_pair(api: &auth_service::AuthApi)-> SetupTokenPairOutput {
    let email = format!("{}@login.com", Uuid::new_v4().to_string());
    let id = insert_user(&email, "password").await;
    let res = api.login(&email, "password").await.unwrap();

    let access_token = res.access_token;
    let refresh_token = res.refresh_token;

    SetupTokenPairOutput {
        email,
        user_id: id,
        password: "password".to_string(),
        access_token,
        refresh_token,
    }
}

pub async fn get_api() -> auth_service::AuthApi {
    auth_service::AuthApi::from_env().await.unwrap()
}

pub async fn insert_user(email: &str, password: &str) -> String {
    let pool = PgPool::connect(&std::env::var("AUTH_SERVICE_DB_CONN_STR").unwrap())
        .await
        .unwrap();

    let result = sqlx::query("INSERT INTO pastureen_user (email, password) VALUES ($1, $2) RETURNING id")
        .bind(email)
        .bind(password)
        .fetch_one(&pool)
        .await
        .unwrap();
    let id:Uuid = result.get("id");
    id.to_string()
}

pub async fn delete_user(id: &str) {
    let pool = PgPool::connect(&std::env::var("AUTH_SERVICE_DB_CONN_STR").unwrap())
        .await
        .unwrap();

    sqlx::query("DELETE FROM pastureen_user WHERE id = $1::uuid")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();
}

pub fn get_expired_access_token(email: &str) -> String {
    encode(
        &Header::default(),
        &Claims {
            sub: email.to_string(),
            token_type: "access".to_string(),
            iat: 0,
            exp: 0,
            id: Uuid::new_v4().to_string()
        },
        &EncodingKey::from_secret(
            std::env::var("AUTH_SERVICE_SECRET").unwrap().as_bytes(),
        ),
    )
    .unwrap()
}

pub fn get_expired_refresh_token(email: &str) -> String {
    encode(
        &Header::default(),
        &Claims {
            sub: email.to_string(),
            token_type: "access".to_string(),
            iat: 0,
            exp: 0,
            id: Uuid::new_v4().to_string()
        },
        &EncodingKey::from_secret(
            std::env::var("AUTH_SERVICE_SECRET").unwrap().as_bytes(),
        ),
    )
    .unwrap()
}

pub fn decode_token(token: &str) -> Claims {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(std::env::var("AUTH_SERVICE_SECRET").unwrap().as_bytes()),
        &Validation::default(),
    )
    .unwrap();
    token_data.claims
}
