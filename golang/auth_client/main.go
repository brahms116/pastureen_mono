package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"pastureen/auth-models"
)

func handle200Response(response *http.Response, v any) error {
	defer response.Body.Close()
	if response.StatusCode != 200 {
		content, err := io.ReadAll(response.Body)
		if err != nil {
			return err
		}
		if len(content) == 0 {
			return errors.New(fmt.Sprintf("Response: %v+", response))
		}
		return errors.New(string(content))
	}
	return json.NewDecoder(response.Body).Decode(v)
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
	err = handle200Response(response, &user)
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
	err = handle200Response(response, &tokenPair)
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
	err = handle200Response(response, &tokenPair)
	return tokenPair.TokenPair, err
}
