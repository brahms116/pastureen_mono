package client

import (
	models "github.com/brahms116/pastureen_mono/golang/auth_models"
)

type Credentials struct {
	Email        string
	Password     string
	AuthEndpoint string
}

func NewCredentials(
	email string,
	password string,
	authEndpoint string,
) Credentials {
	return Credentials{
		Email:        email,
		Password:     password,
		AuthEndpoint: authEndpoint,
	}
}

func (c Credentials) Login() (TokenCredentials, error) {
	requestPayload := models.LoginRequest{
		Email:    c.Email,
		Password: c.Password,
	}
	tokenPair, err := loginApi(c.AuthEndpoint, requestPayload)
	if err != nil {
		return TokenCredentials{}, err
	}
	result := TokenCredentials{
		AccessToken:  tokenPair.AccessToken,
		RefreshToken: tokenPair.RefreshToken,
		AuthEndpoint: c.AuthEndpoint,
	}
	return result, nil
}

type TokenCredentials struct {
	AccessToken  string
	RefreshToken string
	AuthEndpoint string
}

func NewTokenCredentials(
	accessToken string,
	refreshToken string,
	authEndpoint string,
) TokenCredentials {
	return TokenCredentials{
		AccessToken:  accessToken,
		RefreshToken: refreshToken,
		AuthEndpoint: authEndpoint,
	}
}

func (c TokenCredentials) ToAccessCredentials() AccessCredentials {
	return AccessCredentials{
		AccessToken:  c.AccessToken,
		AuthEndpoint: c.AuthEndpoint,
	}
}

func (c TokenCredentials) ToRefreshCredentials() RefreshCredentials {
	return RefreshCredentials{
		RefreshToken: c.RefreshToken,
		AuthEndpoint: c.AuthEndpoint,
	}
}

func (c TokenCredentials) RefreshTokens() (TokenCredentials, error) {
	return c.ToRefreshCredentials().RefreshTokens()
}

func (c TokenCredentials) GetUser() (models.User, error) {
	return c.ToAccessCredentials().GetUser()
}

type AccessCredentials struct {
	AccessToken  string
	AuthEndpoint string
}

func NewAccessCredentials(
	accessToken string,
	authEndpoint string,
) AccessCredentials {
	return AccessCredentials{
		AccessToken:  accessToken,
		AuthEndpoint: authEndpoint,
	}
}

func (c AccessCredentials) GetUser() (models.User, error) {
	return getUserApi(c.AuthEndpoint, c.AccessToken)
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
	tokenPair, err := refreshTokenApi(c.AuthEndpoint, c.RefreshToken)
	if err != nil {
		return TokenCredentials{}, err
	}
	result := TokenCredentials{
		AccessToken:  tokenPair.AccessToken,
		RefreshToken: tokenPair.RefreshToken,
		AuthEndpoint: c.AuthEndpoint,
	}
	return result, nil
}
