package client

import (
	"bytes"
	"encoding/json"
	"github.com/brahms116/pastureen_mono/golang/auth_models"
	httpUtils "github.com/brahms116/pastureen_mono/golang/http_utils"
	"net/http"
)

func GetUserFromTokenCredentials(credentials models.TokenCredentials) (models.User, error) {
	return GetUser(credentials.Endpoint, credentials.AccessToken)
}

func GetUserFromAccessCredentials(credentials models.AccessCredentials) (models.User, error) {
	return GetUser(credentials.Endpoint, credentials.AccessToken)
}

func GetUserFromCredentials(credentials models.Credentials) (models.User, error) {
	tokenCredentials, err := Login(credentials)
	if err != nil {
		return models.User{}, err
	}
	return GetUserFromTokenCredentials(tokenCredentials)
}

func GetUser(endpoint string, accessToken string) (models.User, error) {
	request, err := http.NewRequest("GET", endpoint+"/user", nil)
	if err != nil {
		return models.User{}, err
	}
	request.Header.Set("Authorization", "Bearer "+accessToken)
	response, err := http.DefaultClient.Do(request)
	if err != nil {
		return models.User{}, err
	}
	var user models.GetUserReponse
	err = httpUtils.HandleResponse(response, &user)
	return user.User, err
}

func RefreshToken(endpoint string, refreshToken string) (models.TokenCredentials, error) {
	tokens, err := getRefreshToken(endpoint, refreshToken)
	if err != nil {
		return models.TokenCredentials{}, err
	}
	return models.TokenCredentials{
		AccessToken:  tokens.AccessToken,
		RefreshToken: tokens.RefreshToken,
		Endpoint:     endpoint,
	}, nil
}

func RefreshTokenWithAccessCredentials(credentials models.AccessCredentials, refreshToken string) (models.TokenCredentials, error) {
	return RefreshToken(credentials.Endpoint, refreshToken)
}

func RefreshTokenWithCredentials(credentials models.TokenCredentials) (models.TokenCredentials, error) {
	return RefreshToken(credentials.Endpoint, credentials.RefreshToken)
}

// Exported wrapper around inner login function for easier use
func Login(credentials models.Credentials) (models.TokenCredentials, error) {
	tokenPair, err := login(credentials.Endpoint, models.LoginRequest{
		Email:    credentials.Email,
		Password: credentials.Password,
	})
	if err != nil {
		return models.TokenCredentials{}, err
	}
	return models.TokenCredentials{
		AccessToken:  tokenPair.AccessToken,
		RefreshToken: tokenPair.RefreshToken,
		Endpoint:     credentials.Endpoint,
	}, nil
}

// Private functions as primitives with api contract types

func login(endpoint string, loginRequest models.LoginRequest) (models.TokenPair, error) {
	body, err := json.Marshal(loginRequest)
	if err != nil {
		return models.TokenPair{}, err
	}
	response, err := http.Post(endpoint+"/token", "application/json", bytes.NewBuffer(body))
	if err != nil {
		return models.TokenPair{}, err
	}
	var tokenPair models.TokenPairResponse
	err = httpUtils.HandleResponse(response, &tokenPair)
	return tokenPair.TokenPair, err
}

func getRefreshToken(endpoint string, refreshToken string) (models.TokenPair, error) {
	request, err := http.NewRequest("GET", endpoint+"/token", nil)
	if err != nil {
		return models.TokenPair{}, err
	}
	request.Header.Set("Authorization", "Bearer "+refreshToken)
	response, err := http.DefaultClient.Do(request)
	if err != nil {
		return models.TokenPair{}, err
	}
	var tokenPair models.TokenPairResponse
	err = httpUtils.HandleResponse(response, &tokenPair)
	return tokenPair.TokenPair, err
}
