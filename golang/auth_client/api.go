package client

import (
	"bytes"
	"encoding/json"
	"github.com/brahms116/pastureen_mono/golang/auth_models"
	httpUtils "github.com/brahms116/pastureen_mono/golang/http_utils"
	"net/http"
)

const AUTH_SERVICE_PATH = "/auth"

func getUserApi(endpoint string, accessToken string) (models.User, error) {
	authEndpoint := endpoint + AUTH_SERVICE_PATH
	request, err := http.NewRequest("GET", authEndpoint+"/user", nil)
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

func refreshTokenApi(endpoint string, refreshToken string) (models.TokenPair, error) {
	authEndpoint := endpoint + AUTH_SERVICE_PATH
	request, err := http.NewRequest("GET", authEndpoint+"/token", nil)
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

func loginApi(endpoint string, loginRequest models.LoginRequest) (models.TokenPair, error) {
	authEndpoint := endpoint + AUTH_SERVICE_PATH
	body, err := json.Marshal(loginRequest)
	if err != nil {
		return models.TokenPair{}, err
	}
	response, err := http.Post(authEndpoint+"/token", "application/json", bytes.NewBuffer(body))
	if err != nil {
		return models.TokenPair{}, err
	}
	var tokenPair models.TokenPairResponse
	err = httpUtils.HandleResponse(response, &tokenPair)
	return tokenPair.TokenPair, err
}
