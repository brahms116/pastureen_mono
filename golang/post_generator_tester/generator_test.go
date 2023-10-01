package main

import (
	"bytes"
	_ "embed"
	"encoding/json"
	"errors"
	"io"
	"io/ioutil"
	"net/http"
	"os"
	"testing"
)

//go:embed test_post.md
var testPost string

type Config struct {
	PostGeneratorUrl string
	AuthUrl   string
	Username         string
	Password         string
}

const ENV_PREFIX = "POST_GENERATOR_TESTER_"

func ConfigFromEnv() Config {
	generatorUrl := os.Getenv(ENV_PREFIX + "POST_GENERATOR_URL")
	authServiceUrl := os.Getenv(ENV_PREFIX + "AUTH_URL")
	username := os.Getenv(ENV_PREFIX + "USERNAME")
	password := os.Getenv(ENV_PREFIX + "PASSWORD")
	return Config{generatorUrl, authServiceUrl, username, password}
}

type GeneratePostRequest struct {
	MarkdownString string `json:"markdown"`
}

type PostMeta struct {
	Title string   `json:"title"`
	Slug  string   `json:"slug"`
	Tags  []string `json:"tags"`
	Date  string   `json:"date"`
}

type RenderedPost struct {
	PostMeta PostMeta `json:"meta"`
	PostHtml string   `json:"postHtml"`
}

type GeneratePostResponse struct {
	GeneratedPost RenderedPost `json:"generatedPost"`
}

type LoginRequest struct {
	Email    string `json:"email"`
	Password string `json:"password"`
}

type TokenPair struct {
	AcceessToken string `json:"accessToken"`
	RefreshToken string `json:"refreshToken"`
}

type LoginResponse struct {
	TokenPair TokenPair `json:"tokenPair"`
}

func GetTokenPair(authServiceUrl string, username string, password string) (LoginResponse, error) {
	loginRequest := LoginRequest{username, password}
	jsonBody, err := json.Marshal(loginRequest)
	if err != nil {
		return LoginResponse{}, err
	}

	resp, err := http.Post(authServiceUrl+"/token", "application/json", bytes.NewBuffer(jsonBody))

	if err != nil {
		return LoginResponse{}, err
	}
	defer resp.Body.Close()

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return LoginResponse{}, err
	}

	if resp.StatusCode != 200 {
		return LoginResponse{}, errors.New(string(body))
	}

	var loginResponse LoginResponse
	err = json.Unmarshal(body, &loginResponse)

	if err != nil {
		return LoginResponse{}, err
	}
	return loginResponse, nil
}

func TestGenerator(t *testing.T) {
	config := ConfigFromEnv()

	loginResponse, err := GetTokenPair(config.AuthUrl, config.Username, config.Password)
	if err != nil {
		t.Fatal(err)
	}

	read, write := io.Pipe()

	go func() {
		defer write.Close()
		encoder := json.NewEncoder(write)
		encoder.SetEscapeHTML(false)
		encoder.Encode(GeneratePostRequest{testPost})
	}()

	request, err := http.NewRequest("POST", config.PostGeneratorUrl, read)
	if err != nil {
		t.Fatal(err)
	}
	request.Header.Add("Authorization", "Bearer "+loginResponse.TokenPair.AcceessToken)

	resp, err := http.DefaultClient.Do(request)
	if err != nil {
		t.Error(err)
		return
	}

	if resp.StatusCode != 200 {
		t.Errorf("Uexpected status code: %d", resp.StatusCode)
		body, err := ioutil.ReadAll(resp.Body)
		if err != nil {
			t.Fatalf("Error reading response body of error: %v", err)
			return
		}
		t.Fatalf("Unexpected status code: %d,\nBody: %s\n", resp.StatusCode, body)
	}

	defer resp.Body.Close()
	decoder := json.NewDecoder(resp.Body)
	var generatePostResponse GeneratePostResponse
	err = decoder.Decode(&generatePostResponse)
	if err != nil {
		t.Error(err)
		return
	}

	expectedTitle := "My post title"
	expectedSlug := "my-post-title"
	expectedTags := []string{"rust", "tech"}

	resultTitle := generatePostResponse.GeneratedPost.PostMeta.Title
	resultSlug := generatePostResponse.GeneratedPost.PostMeta.Slug
	resultTags := generatePostResponse.GeneratedPost.PostMeta.Tags

	if resultTitle != expectedTitle {
		t.Errorf("Unexpected title: %s, expected %s", resultTitle, expectedTitle)
	}

	if resultSlug != expectedSlug {
		t.Errorf("Unexpected slug: %s, expected %s", resultSlug, expectedSlug)
	}

	if len(resultTags) != len(expectedTags) {
		t.Fatalf("Unexpected tags: %v, expected %v", resultTags, expectedTags)
	}

	for i, tag := range resultTags {
		if tag != expectedTags[i] {
			t.Errorf("Unexpected tag: %s, expected %s", tag, expectedTags[i])
		}
	}
}
