use auth_models::*;
use auth_service_models::*;
use reqwest::Client;
use reqwest_utils::*;
use shared_models::*;

pub async fn get_user(endpoint: &str, access_token: &str) -> Result<User, ClientHttpResponseError> {
    let client = Client::new();

    let res = client
        .post(&format!("{}/user", endpoint))
        .bearer_auth(access_token)
        .send()
        .await;

    handle_res::<GetUserResponse>(res).await.map(|res| res.user)
}

pub async fn refresh_token(
    endpoint: &str,
    refresh_token: &str,
) -> Result<TokenPair, ClientHttpResponseError> {
    let client = Client::new();
    let res = client
        .get(&format!("{}/token", endpoint))
        .bearer_auth(refresh_token)
        .send()
        .await;

    handle_res::<TokenPairResponse>(res)
        .await
        .map(|res| res.token_pair)
}

pub async fn login(endpoint: &str, request:&LoginRequest) -> Result<TokenPair, ClientHttpResponseError> {
    let client = Client::new();
    let res = client
        .post(&format!("{}/token", endpoint))
        .json(request)
        .send()
        .await;

    handle_res::<TokenPairResponse>(res)
        .await
        .map(|res| res.token_pair)
}
