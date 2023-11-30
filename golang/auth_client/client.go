package client

import (
	models "github.com/brahms116/pastureen_mono/golang/auth_models"
)

type Credentials struct {
	email        string
	password     string
	authEndpoint string
}

func NewCredentials(
	email string,
	password string,
	authEndpoint string,
) Credentials {
	return Credentials{
		email:        email,
		password:     password,
		authEndpoint: authEndpoint,
	}
}

func (c Credentials) Login() (TokenCredentials, error) {
	requestPayload := models.LoginRequest{
		Email:    c.email,
		Password: c.password,
	}
	tokenPair, err := login(c.authEndpoint, requestPayload)
	if err != nil {
		return TokenCredentials{}, err
	}
	result := TokenCredentials{
		AccessToken:  tokenPair.AccessToken,
		RefreshToken: tokenPair.RefreshToken,
		authEndpoint: c.authEndpoint,
	}
	return result, nil
}

type TokenCredentials struct {
	AccessToken  string
	RefreshToken string
	authEndpoint string
}

func NewTokenCredentials(
	accessToken string,
	refreshToken string,
	authEndpoint string,
) TokenCredentials {
	return TokenCredentials{
		AccessToken:  accessToken,
		RefreshToken: refreshToken,
		authEndpoint: authEndpoint,
	}
}

func (c TokenCredentials) RefreshTokens() (TokenCredentials, error) {
	tokenPair, err := refreshToken(c.authEndpoint, c.RefreshToken)
	if err != nil {
		return TokenCredentials{}, err
	}
	result := NewTokenCredentials(
		tokenPair.AccessToken,
		tokenPair.RefreshToken,
		c.authEndpoint,
	)
	return result, nil
}

func (c TokenCredentials) GetUser() (models.User, error) {
	return getUser(c.authEndpoint, c.AccessToken)
}

type AccessCredentials struct {
	accessToken  string
	authEndpoint string
}

func NewAccessCredentials(
	accessToken string,
	authEndpoint string,
) AccessCredentials {
	return AccessCredentials{
		accessToken:  accessToken,
		authEndpoint: authEndpoint,
	}
}

func (c AccessCredentials) GetUser() (models.User, error) {
	return getUser(c.authEndpoint, c.accessToken)
}

type RefreshCredentials struct {
	RefreshToken string
	AuthEndpoint string
}

func NewRefreshCredentials(
	refreshToken string,
	authEndpoint string,
) RefreshCredentials {
	return RefreshCredentials{
		RefreshToken: refreshToken,
		AuthEndpoint: authEndpoint,
	}
}

func (c RefreshCredentials) RefreshTokens() (TokenCredentials, error) {
	tokenPair, err := refreshToken(c.AuthEndpoint, c.RefreshToken)
	if err != nil {
		return TokenCredentials{}, err
	}
	result := NewTokenCredentials(
		tokenPair.AccessToken,
		tokenPair.RefreshToken,
		c.AuthEndpoint,
	)
	return result, nil
}
