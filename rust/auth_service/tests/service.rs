mod common;
use common::*;

#[tokio::test]
async fn login() {
    let api = get_api().await;
    let SetupTokenPairOutput {
        user_id,
        access_token,
        refresh_token,
        ..
    } = setup_token_pair(&api).await;

    let access_token = decode_token(&access_token);
    assert_eq!(access_token.sub, user_id);
    assert_eq!(access_token.token_type, "access");

    let refresh_token = decode_token(&refresh_token);
    assert_eq!(refresh_token.sub, user_id);
    assert_eq!(refresh_token.token_type, "refresh");

    delete_user(&user_id).await;
}

#[tokio::test]
async fn get_user() {
    let api = get_api().await;
    let SetupTokenPairOutput {
        user_id,
        access_token,
        refresh_token,
        email,
        ..
    } = setup_token_pair(&api).await;

    let user = api.get_user(&access_token).await.unwrap();
    assert_eq!(user.id, user_id);
    assert_eq!(user.email, email);

    // Blatantly incorrect token
    let incorrect = api.get_user(&refresh_token).await;
    assert!(incorrect.is_err());
    assert!(matches!(
        incorrect.unwrap_err(),
        auth_service::AuthApiError::InvalidToken
    ));

    // Expired token
    let expired_token = get_expired_access_token("test2@login.com");
    let expired = api.get_user(&expired_token).await;
    assert!(expired.is_err());
    assert!(matches!(
        expired.unwrap_err(),
        auth_service::AuthApiError::InvalidToken
    ));

    delete_user(&user_id).await;
}

#[tokio::test]
async fn refresh() {
    let api = get_api().await;
    let SetupTokenPairOutput {
        user_id,
        access_token,
        refresh_token,
        email,
        ..
    } = setup_token_pair(&api).await;

    // incorrect refresh token
    let incorrect = api.refresh(&access_token).await;
    assert!(incorrect.is_err());
    assert!(matches!(
        incorrect.unwrap_err(),
        auth_service::AuthApiError::InvalidToken
    ));

    // expired refresh token
    let expired_token = get_expired_refresh_token(&email);
    let expired = api.refresh(&expired_token).await;
    assert!(expired.is_err());
    assert!(matches!(
        expired.unwrap_err(),
        auth_service::AuthApiError::InvalidToken
    ));

    // correct refresh token
    let correct_res = api.refresh(&refresh_token).await.unwrap();
    let access_token = decode_token(&correct_res.access_token);
    assert_eq!(access_token.sub, user_id);

    // using the old refresh token
    let res2 = api.refresh(&refresh_token).await;
    assert!(res2.is_err());
    assert!(matches!(
        res2.unwrap_err(),
        auth_service::AuthApiError::InvalidToken
    ));

    // using the new refresh token
    let res3 = api.refresh(&correct_res.refresh_token).await;
    assert!(res3.is_err());
    assert!(matches!(
        res3.unwrap_err(),
        auth_service::AuthApiError::InvalidToken
    ));

    delete_user(&user_id).await;
}
