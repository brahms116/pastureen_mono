use std::env;

use auth_client::*;
use auth_models::*;
use auth_service_models::*;
use shared_models::*;

struct TestConfig {
    pub email: String,
    pub password: String,
    pub url: String,
}

impl TestConfig {
    pub fn from_env() -> Self {
        let email = env::var("ADMIN_EMAIL").unwrap();
        let password = env::var("ADMIN_PASSWORD").unwrap();
        let url = env::var("AUTH_SERVICE_URL").unwrap();

        Self {
            email,
            password,
            url,
        }
    }
}

async fn login_user() -> Result<(TestConfig, TokenPair), ClientHttpResponseError> {
    let config = TestConfig::from_env();

    let login_request = LoginRequest {
        email: config.email.clone(),
        password: config.password.clone(),
    };

    let login_response = login(&config.url, &login_request).await?;

    Ok((config, login_response))
}

#[tokio::test]
async fn test_login() {
    let (_, token_pair) = login_user().await.unwrap();
    assert!(token_pair.access_token != token_pair.refresh_token);
}

#[tokio::test]
async fn test_get_user() {
    let (config, token_pair) = login_user().await.unwrap();
    let user = get_user(&config.url, &token_pair.access_token).await.unwrap();
    assert_eq!(user.email, config.email);
}

#[tokio::test]
async fn test_refresh_token() {
    let (config, token_pair) = login_user().await.unwrap();
    let new_token_pair = refresh_token(&config.url, &token_pair.refresh_token)
        .await
        .unwrap();
    assert!(new_token_pair.access_token != new_token_pair.refresh_token);
    assert!(new_token_pair.access_token != token_pair.access_token);
}


