pub async fn get_api() -> auth_service::AuthApi {
    auth_service::AuthApi::from_env().await.unwrap()
}

