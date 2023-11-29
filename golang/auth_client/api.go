package client

import (
	"bytes"
	"encoding/json"
	"github.com/brahms116/pastureen_mono/golang/auth_models"
	httpUtils "github.com/brahms116/pastureen_mono/golang/http_utils"
	"net/http"
)

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

func RefreshToken(endpoint string, refreshToken string) (models.TokenPair, error) {
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

func Login(endpoint string, loginRequest models.LoginRequest) (models.TokenPair, error) {
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
