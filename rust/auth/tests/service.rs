mod common;
use common::*;
use auth::*;
use auth_models::*;

#[tokio::test]
async fn login() {
    let api = get_auth().await;
    let SetupTokenPairOutput {
        access_token,
        refresh_token,
        email,
        ..
    } = setup_token_pair(&api).await;

    let access_token = decode_token_helper(&access_token);
    assert_eq!(access_token.sub, email);
    assert_eq!(access_token.token_type, TokenType::Access);

    let refresh_token = decode_token_helper(&refresh_token);
    assert_eq!(refresh_token.sub, email);
    assert_eq!(refresh_token.token_type, TokenType::Refresh);

    delete_user(&email).await;
}

#[tokio::test]
async fn get_user() {
    let api = get_auth().await;
    let SetupTokenPairOutput {
        access_token,
        refresh_token,
        email,
        ..
    } = setup_token_pair(&api).await;

    let user = api.get_user(&access_token).await.unwrap();
    assert_eq!(user.email, email);

    // Blatantly incorrect token
    let incorrect = api.get_user(&refresh_token).await;
    assert!(incorrect.is_err());
    assert!(matches!(
        incorrect.unwrap_err(),
        AuthError::InvalidToken
    ));

    // Expired token
    let expired_token = get_expired_access_token("test2@login.com");
    let expired = api.get_user(&expired_token).await;
    assert!(expired.is_err());
    assert!(matches!(
        expired.unwrap_err(),
        AuthError::InvalidToken
    ));

    delete_user(&email).await;
}

#[tokio::test]
async fn refresh() {
    let api = get_auth().await;
    let SetupTokenPairOutput {
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
        AuthError::InvalidToken
    ));

    // expired refresh token
    let expired_token = get_expired_refresh_token(&email);
    let expired = api.refresh(&expired_token).await;
    assert!(expired.is_err());
    assert!(matches!(
        expired.unwrap_err(),
        AuthError::InvalidToken
    ));

    // correct refresh token
    let correct_res = api.refresh(&refresh_token).await.unwrap();
    let access_token = decode_token_helper(&correct_res.access_token);
    assert_eq!(access_token.sub, email);

    // using the old refresh token
    let res2 = api.refresh(&refresh_token).await;
    assert!(res2.is_err());
    assert!(matches!(
        res2.unwrap_err(),
        AuthError::InvalidToken
    ));

    // using the new refresh token
    let res3 = api.refresh(&correct_res.refresh_token).await;
    assert!(res3.is_err());
    assert!(matches!(
        res3.unwrap_err(),
        AuthError::InvalidToken
    ));

    delete_user(&email).await;
}
