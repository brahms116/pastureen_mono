package client

import (
	"io"
	"net/http"
	"os"
	authClient "pastureen/auth-client"
	authModels "pastureen/auth-models"
	blogModels "pastureen/blog-models"
	libModels "pastureen/librarian-models"
	"testing"

	"github.com/google/uuid"
)

const ENV_PREFIX = "LIBRARIAN_CLIENT_TEST_"

type TestConfig struct {
	LibrarianUrl string
	AuthUrl      string
	Email        string
	Password     string
	BlogUrl      string
}

func ConfigFromEnv() TestConfig {
	librarianUrl := os.Getenv(ENV_PREFIX + "LIBRARIAN_URL")
	authServiceUrl := os.Getenv(ENV_PREFIX + "AUTH_URL")
	username := os.Getenv(ENV_PREFIX + "EMAIL")
	password := os.Getenv(ENV_PREFIX + "PASSWORD")
	return TestConfig{
		LibrarianUrl: librarianUrl,
		AuthUrl:      authServiceUrl,
		Email:        username,
		Password:     password,
		BlogUrl:      os.Getenv(ENV_PREFIX + "BLOG_URL"),
	}

}

func login() (TestConfig, string, error) {
	config := ConfigFromEnv()
	loginRequest := authModels.LoginRequest{
		Email:    config.Email,
		Password: config.Password,
	}
	tokens, err := authClient.Login(config.AuthUrl, loginRequest)
	if err != nil {
		return config, "", err
	}
	return config, tokens.AccessToken, nil
}

func TestFlow(t *testing.T) {
	config, accessToken, err := login()
	if err != nil {
		t.Fatal(err)
	}
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
	url, err := UploadPost(config.LibrarianUrl, accessToken, request)
	if err != nil {
		t.Fatal(err)
	}

	if url != "/posts/"+randomSlug+".html" {
		t.Fatalf("Expected url to be /posts/%s.html, got %s", randomSlug, url)
	}
	expectedLocation := config.BlogUrl + url
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
}
