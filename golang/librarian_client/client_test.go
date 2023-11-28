package client

import (
	authClient "github.com/brahms116/pastureen_mono/golang/auth_client"
	authModels "github.com/brahms116/pastureen_mono/golang/auth_models"
	blogModels "github.com/brahms116/pastureen_mono/golang/blog_models"
	libModels "github.com/brahms116/pastureen_mono/golang/librarian_models"
	"io"
	"net/http"
	"os"
	"testing"

	"github.com/google/uuid"
)

type TestConfig struct {
	Email             string
	Password          string
	PastureenEndpoint string
}

func ConfigFromEnv() TestConfig {
	username := os.Getenv("ADMIN_EMAIL")
	password := os.Getenv("ADMIN_PASSWORD")
  pastureenEndpoint := os.Getenv("REVERSE_PROXY_URL")
	return TestConfig{
		Email:        username,
		Password:     password,
    PastureenEndpoint: pastureenEndpoint,
	}
}

func login() (authModels.AuthenticatedApiRequestConfig, error) {
	config := ConfigFromEnv()

	credentials := authModels.NewCredentials(
		config.PastureenEndpoint,
		config.Email,
		config.Password,
	)
	tokens, err := authClient.Login(credentials)
	if err != nil {
		return authModels.AuthenticatedApiRequestConfig{}, err
	}
	return authModels.ApiRequestConfigFromTokenCredentials(tokens), nil
}

func TestFlow(t *testing.T) {

	requestConfig, err := login()
	if err != nil {
		t.Fatal(err)
	}

  blogEndpoint := requestConfig.Endpoint + "/blog"

	randomSlug := uuid.New().String()
	tag1 := uuid.New().String()
	tag2 := uuid.New().String()

	html := "<h1>Test Post - " + randomSlug + "</h1><p>This is a test post</p>"
	request := libModels.CreateNewPostRequest{
		Post: blogModels.Post{
			PostMeta: blogModels.PostMeta{
				Title: "Test Post - " + randomSlug,
				Slug:  randomSlug,
				Tags:  []string{tag1, tag2},
				Date:  "2020-09-18",
			},
			PostHtml: html,
		},
	}
	url, err := UploadPost(requestConfig, request)
	if err != nil {
		t.Fatal(err)
	}

	if url != "/posts/"+randomSlug+".html" {
		t.Fatalf("Expected url to be /posts/%s.html, got %s", randomSlug, url)
	}
	expectedLocation := blogEndpoint + url

	// Retireve the page via a GET request
	confirmReq, err := http.NewRequest("GET", expectedLocation, nil)
	if err != nil {
		t.Fatal(err)
	}
	confirmResp, err := http.DefaultClient.Do(confirmReq)
	if err != nil {
		t.Fatal(err)
	}
	content, err := io.ReadAll(confirmResp.Body)
	if err != nil {
		t.Fatal(err)
	}
	if string(content) != html {
		t.Fatalf("Expected content to be %s, got %s", html, string(content))
	}

	// Try getting link via url
	getResp, err := GetLink(requestConfig.Endpoint, url)
	if err != nil {
		t.Fatal(err)
	}

	if getResp.Url != url {
		t.Fatalf("Expected url to be %s (GetLink), got %s", url, getResp.Url)
	}

	// Try getting a wrong link
	value, err := GetLink(requestConfig.Endpoint, tag1)
	if err != nil {
		t.Fatal(err)
	}
	if value != nil {
		t.Fatalf("Expected value to be nil (GetLink), got %s", value)
	}

	// Try searching for the post via title
	searchReq := libModels.QueryLinksRequest{
		TitleQuery: randomSlug,
	}

	searchResp, err := SearchLinks(requestConfig.Endpoint, searchReq)
	if err != nil {
		t.Fatal(err)
	}
	if len(searchResp) != 1 {
		t.Fatalf("Expected 1 result (search by title), got %d", len(searchResp))
	}
	if searchResp[0].Url != url {
		t.Fatalf("Expected url to be %s (search by title), got %s", url, searchResp[0].Url)
	}

	// Try searching by tag1
	searchReq = libModels.QueryLinksRequest{
		Tags: []string{tag1},
	}

	searchResp, err = SearchLinks(requestConfig.Endpoint, searchReq)
	if err != nil {
		t.Fatal(err)
	}
	if len(searchResp) != 1 {
		t.Fatalf("Expected 1 result (search by tag1), got %d", len(searchResp))
	}
	if searchResp[0].Url != url {
		t.Fatalf("Expected url to be %s (search by tag1), got %s", url, searchResp[0].Url)
	}

	// Try searching by tag2
	searchReq = libModels.QueryLinksRequest{
		Tags: []string{tag2},
	}

	searchResp, err = SearchLinks(requestConfig.Endpoint, searchReq)
	if err != nil {
		t.Fatal(err)
	}
	if len(searchResp) != 1 {
		t.Fatalf("Expected 1 result (search by tag2), got %d", len(searchResp))
	}
	if searchResp[0].Url != url {
		t.Fatalf("Expected url to be %s (search by tag2), got %s", url, searchResp[0].Url)
	}
}
