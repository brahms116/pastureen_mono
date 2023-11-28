package client

import (
	_ "embed"
	authClient "github.com/brahms116/pastureen_mono/golang/auth_client"
	authModels "github.com/brahms116/pastureen_mono/golang/auth_models"
	publisherModels "github.com/brahms116/pastureen_mono/golang/publisher_models"
	"os"
	"testing"
)

//go:embed test_post.md
var TEST_POST string

type TestConfig struct {
	PastureenEndpoint string
	Email             string
	Password          string
}

func ConfigFromEnv() TestConfig {

	pastureenEndpoint := os.Getenv("REVERSE_PROXY_URL")
	email := os.Getenv("ADMIN_EMAIL")
	password := os.Getenv("ADMIN_PASSWORD")
	return TestConfig{
		PastureenEndpoint: pastureenEndpoint,
		Email:             email,
		Password:          password,
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

func TestPublish(t *testing.T) {
	apiRequestConfig, err := login()
	if err != nil {
		t.Fatal(err)
	}

	generateRequest := publisherModels.GeneratePostRequest{
		MarkdownString: TEST_POST,
	}

	post, err := Generate(apiRequestConfig, generateRequest)
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
