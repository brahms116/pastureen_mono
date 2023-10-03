package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"io"
	"net/http"
	"pastureen/auth-models"
)

func getErrorFromReader(reader io.Reader) error {
	content, err := io.ReadAll(reader)
	if err != nil {
		return err
	}
	return errors.New(string(content))
}

func handle200Response(response *http.Response, v any) error {
	defer response.Body.Close()
	if response.StatusCode != 200 {
		return getErrorFromReader(response.Body)
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
	var user models.User
	err = handle200Response(response, &user)
	return user, err
}

func RefreshToken(endpoint string, refresh string) (models.TokenPair, error) {
	request, err := http.NewRequest("GET", endpoint+"/token", nil)
	if err != nil {
		return models.TokenPair{}, err
	}
	request.Header.Set("Authorization", "Bearer "+refresh)
	response, err := http.DefaultClient.Do(request)
	if err != nil {
		return models.TokenPair{}, err
	}
	var tokenPair models.TokenPair
	err = handle200Response(response, &tokenPair)
	return tokenPair, err
}

func Login(endpoint string, loginRequest models.LoginRequest) (models.TokenPair, error) {
	body, err := json.Marshal(loginRequest)
	if err != nil {
		return models.TokenPair{}, err
	}
	response, err := http.Post(endpoint+"/login", "application/json", bytes.NewBuffer(body))
	if err != nil {
		return models.TokenPair{}, err
	}
	var tokenPair models.TokenPair
	err = handle200Response(response, &tokenPair)
	return tokenPair, err
}
