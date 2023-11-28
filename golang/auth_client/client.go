package client

import (
	"github.com/brahms116/pastureen_mono/golang/auth_models"
)

func GetUser(config models.AuthenticatedApiRequestConfig) (models.User, error) {
	return getUserApi(config.Endpoint, config.AccessToken)
}

func Login(credentials models.Credentials) (models.TokenCredentials, error) {
	requestPayload := models.LoginRequest{
		Email:    credentials.Email,
		Password: credentials.Password,
	}
	tokenPair, err := loginApi(credentials.Endpoint, requestPayload)
	if err != nil {
		return models.TokenCredentials{}, err
	}
	result := models.TokenCredentials{
		AccessToken:  tokenPair.AccessToken,
		RefreshToken: tokenPair.RefreshToken,
		Endpoint:     credentials.Endpoint,
	}
	return result, nil
}

func RefreshToken(credentials models.TokenCredentials) (models.TokenCredentials, error) {
	tokenPair, err := refreshTokenApi(credentials.Endpoint, credentials.RefreshToken)
	if err != nil {
		return models.TokenCredentials{}, err
	}
	result := models.TokenCredentials{
		AccessToken:  tokenPair.AccessToken,
		RefreshToken: tokenPair.RefreshToken,
		Endpoint:     credentials.Endpoint,
	}
	return result, nil
}
