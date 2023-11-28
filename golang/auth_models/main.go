package models

// Contracts
type User struct {
	Fname string `json:"fname"`
	Lname string `json:"lname"`
	Email string `json:"email"`
}

type Claims struct {
	Sub       string `json:"sub"`
	Exp       int64  `json:"exp"`
	Iat       int64  `json:"iat"`
	TokenType string `json:"tokenType"`
	Id        string `json:"id"`
}

type TokenPair struct {
	AccessToken  string `json:"accessToken"`
	RefreshToken string `json:"refreshToken"`
}

type GetUserReponse struct {
	User User `json:"user"`
}

type LoginRequest struct {
	Email    string `json:"email"`
	Password string `json:"password"`
}

type TokenPairResponse struct {
	TokenPair TokenPair `json:"tokenPair"`
}

// Client types
type Credentials struct {
	Endpoint string
	Email    string
	Password string
}

func NewCredentials(endpoint string, email string, password string) Credentials {
	return Credentials{
		Endpoint: endpoint,
		Email:    email,
		Password: password,
	}
}

type TokenCredentials struct {
	Endpoint     string
	AccessToken  string
	RefreshToken string
}

type AuthenticatedApiRequestConfig struct {
	Endpoint    string
	AccessToken string
}

func NewApiRequestConfig(endpoint string, accessToken string) AuthenticatedApiRequestConfig {
	return AuthenticatedApiRequestConfig{
		Endpoint:    endpoint,
		AccessToken: accessToken,
	}
}

func ApiRequestConfigFromTokenCredentials(credentials TokenCredentials) AuthenticatedApiRequestConfig {
	return AuthenticatedApiRequestConfig{
		Endpoint:    credentials.Endpoint,
		AccessToken: credentials.AccessToken,
	}
}
