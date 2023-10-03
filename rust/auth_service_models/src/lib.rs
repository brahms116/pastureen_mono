use auth_models::*;
use serde::{Deserialize, Serialize};

// SERVICE_CONTRACTS

/// A response from the Auth Service to a request for a user
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserResponse {
    /// The the requested user
    pub user: User,
}

/// Request to retrieve a token pair from logging in
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Response from the Auth Service to a request for a token pair
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPairResponse {
    pub token_pair: TokenPair,
}
