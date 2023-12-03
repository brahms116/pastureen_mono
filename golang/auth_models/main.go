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


// Client structs
type Credentials struct {
	Email    string
	Password string
	Endpoint string
}

type TokenCredentials struct {
	AccessToken  string
	RefreshToken string
	Endpoint     string
}

type AccessCredentials struct {
	AccessToken string
	Endpoint    string
}
