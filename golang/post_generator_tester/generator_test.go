package main

import (
	_ "embed"
	"encoding/json"
	"io"
	"net/http"
	"os"
	"testing"
)

//go:embed test_post.md
var testPost string

type Config struct {
	PostGeneratorUrl string
	AuthServiceUrl   string
	Username         string
	Password         string
}

const ENV_PREFIX = "POST_GENERATOR_TESTER_"

func ConfigFromEnv() Config {
	generatorUrl := os.Getenv(ENV_PREFIX + "POST_GENERATOR_URL")
	authServiceUrl := os.Getenv(ENV_PREFIX + "AUTH_SERVICE_URL")
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

func TestGenerator(t *testing.T) {
	config := ConfigFromEnv()
	read, write := io.Pipe()
	go func() {
		defer write.Close()
		encoder := json.NewEncoder(write)
		encoder.SetEscapeHTML(false)
		encoder.Encode(GeneratePostRequest{testPost})
	}()
	resp, err := http.Post(config.PostGeneratorUrl, "application/json", read)
	if err != nil {
		t.Error(err)
		return
	}
	defer resp.Body.Close()
	decoder := json.NewDecoder(resp.Body)
	var generatePostResponse GeneratePostResponse
	err = decoder.Decode(&generatePostResponse)
	if err != nil {
		t.Error(err)
		return
	}

	if resp.StatusCode != 200 {
		t.Errorf("Unexpected status code: %d", resp.StatusCode)
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
