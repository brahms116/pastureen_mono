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
	Email               string
	Password            string
	AuthServiceEndpoint string
	PublisherEndpoint   string
}

func ConfigFromEnv() TestConfig {

	authServiceEndpoint := os.Getenv("AUTH_SERVICE_URL")
	publisherEndpoint := os.Getenv("PUBLISHER_URL")
	email := os.Getenv("ADMIN_EMAIL")
	password := os.Getenv("ADMIN_PASSWORD")
	return TestConfig{
		Email:               email,
		Password:            password,
		AuthServiceEndpoint: authServiceEndpoint,
		PublisherEndpoint:   publisherEndpoint,
	}
}

func login() (publisherModels.Credentials, error) {
	config := ConfigFromEnv()

	authCreds, err := authClient.Login(authModels.Credentials{
		Email:    config.Email,
		Password: config.Password,
		Endpoint: config.AuthServiceEndpoint,
	})

	if err != nil {
		return publisherModels.Credentials{}, err
	}
	return publisherModels.Credentials{
		AccessToken: authCreds.AccessToken,
		Endpoint:    config.PublisherEndpoint,
	}, nil
}

func TestPublish(t *testing.T) {
	accessCreds, err := login()
	if err != nil {
		t.Fatal(err)
	}

	generateRequest := publisherModels.GeneratePostRequest{
		MarkdownString: TEST_POST,
	}

	post, err := GeneratePost(accessCreds.Endpoint, accessCreds.AccessToken, generateRequest)
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
