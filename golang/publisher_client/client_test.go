package client

import (
	_ "embed"
	"os"
	authClient "pastureen/auth-client"
	authModels "pastureen/auth-models"
	publisherModels "pastureen/publisher-models"
	"testing"
)

const ENV_PREFIX = "PUBLISHER_CLIENT_TEST_"

//go:embed test_post.md
var TEST_POST string

type TestConfig struct {
	PublisherUrl string
	AuthUrl      string
	Email        string
	Password     string
}

func ConfigFromEnv() TestConfig {
	publisherUrl := os.Getenv(ENV_PREFIX + "PUBLISHER_URL")
	authServiceUrl := os.Getenv(ENV_PREFIX + "AUTH_URL")
	username := os.Getenv(ENV_PREFIX + "EMAIL")
	password := os.Getenv(ENV_PREFIX + "PASSWORD")
	return TestConfig{publisherUrl, authServiceUrl, username, password}
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

func TestPublish(t *testing.T) {
	config, token, err := login()
	if err != nil {
		t.Fatal(err)
	}

	generateRequest := publisherModels.GeneratePostRequest{
		MarkdownString: TEST_POST,
	}

	post, err := Generate(config.PublisherUrl, token, generateRequest)
	if err != nil {
		t.Fatal(err)
	}

	expectedTitle := "My post title"
	expectedSlug := "my-post-title"
	expectedTags := []string{"rust", "tech"}

	resultTitle := post.PostMeta.Title
	resultSlug := post.PostMeta.Slug
	resultTags := post.PostMeta.Tags

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
