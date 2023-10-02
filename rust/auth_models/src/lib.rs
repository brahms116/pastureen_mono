use serde::{Deserialize, Serialize};


// DOMAIN_MODELS

/// A user representation in the Auth Api
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    /// The first name of the user
    pub fname: String,
    /// The last name of the user
    pub lname: String,
    /// A unique email address for the user
    pub email: String,
}

/// A representation of the claims the tokens provided by the Auth Api
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    /// The subject of the token, this is the user email
    pub sub: String,
    /// The expiration time of the token
    pub exp: u64,
    /// The time the token was issued
    pub iat: u64,
    /// The type of the token
    pub token_type: TokenType,
    /// A unique identifier for the token, this is currently just to make tokens unique and serves no other purpose
    pub id: String,
}

/// Type of token issued by the Auth Api
#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd, Clone)]
pub enum TokenType {
    #[serde(rename = "ACCESS")]
    Access,
    #[serde(rename = "REFRESH")]
    Refresh,
}

/// A pair of tokens, an access token and a refresh token
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

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

