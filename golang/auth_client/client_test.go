package client

import (
	"os"
	"github.com/brahms116/pastureen_mono/golang/auth_models"
	"testing"
)


type testConfig struct {
	Email    string
	Password string
	Endpoint string
}

func getTestConfig() testConfig {
	return testConfig{
		Email:    os.Getenv("ADMIN_EMAIL"),
		Password: os.Getenv("ADMIN_PASSWORD"),
		Endpoint: os.Getenv("AUTH_SERVICE_URL"),
	}
}

func testLogin() (testConfig, models.TokenPair, error) {
	config := getTestConfig()
	loginRequest := models.LoginRequest{
		Email:    config.Email,
		Password: config.Password,
	}

	tokens, err := login(config.Endpoint, loginRequest)
	return config, tokens, err
}

func TestLogin(t *testing.T) {
	_, tokens, err := testLogin()
	if err != nil {
		t.Fatal(err)
	}
	if tokens.AccessToken == "" {
		t.Error("Access token is empty")
	}
	if tokens.RefreshToken == "" {
		t.Error("Refresh token is empty")
	}
}

func TestGetUser(t *testing.T) {
	config, tokens, err := testLogin()
	if err != nil {
		t.Fatal(err)
	}
	user, err := GetUser(config.Endpoint, tokens.AccessToken)
	if err != nil {
		t.Fatal(err)
	}
	if user.Email != config.Email {
		t.Error("Email does not match")
	}
}

func TestRefreshToken(t *testing.T) {
	config, tokens, err := testLogin()
	if err != nil {
		t.Fatal(err)
	}
	newTokens, err := RefreshToken(config.Endpoint, tokens.RefreshToken)
	if err != nil {
		t.Fatal(err)
	}
	if newTokens.AccessToken == "" {
		t.Error("Access token is empty")
	}
	if newTokens.RefreshToken == "" {
		t.Error("Refresh token is empty")
	}
	if newTokens.AccessToken == tokens.AccessToken {
		t.Error("Access token is the same")
	}
}
