package main

import (
	"log"
	"os"
	authClient "github.com/brahms116/pastureen_mono/golang/auth_client"
	authModels "github.com/brahms116/pastureen_mono/golang/auth_models"
	libClient "github.com/brahms116/pastureen_mono/golang/librarian_client"
	libModels "github.com/brahms116/pastureen_mono/golang/librarian_models"
	pubClient "github.com/brahms116/pastureen_mono/golang/publisher_client"
	pubModels "github.com/brahms116/pastureen_mono/golang/publisher_models"
	"sync"
)

const ENV_PREFIX = "AUTHOR_"

type AuthorConfig struct {
	PublisherUrl  string
	LibrarianUrl  string
	AuthUrl       string
	AdminEmail    string
	AdminPassword string
}

func ConfigFromEnv() AuthorConfig {
	return AuthorConfig{
		PublisherUrl:  os.Getenv(ENV_PREFIX + "PUBLISHER_URL"),
		LibrarianUrl:  os.Getenv(ENV_PREFIX + "LIBRARIAN_URL"),
		AuthUrl:       os.Getenv(ENV_PREFIX + "AUTH_URL"),
		AdminEmail:    os.Getenv(ENV_PREFIX + "ADMIN_EMAIL"),
		AdminPassword: os.Getenv(ENV_PREFIX + "ADMIN_PASSWORD"),
	}
}

const MAX_WORKERS = 10

func author(path string, config AuthorConfig, accessToken string) {
	contents, err := os.ReadFile(path)
	if err != nil {
		log.Printf("Error opening file %s: %s", path, err)
		return
	}

	log.Printf("Authoring file %s", path)
	stringContents := string(contents)

	request := pubModels.GeneratePostRequest{
		MarkdownString: stringContents,
	}

	log.Printf("Generating html for file %s", path)
	post, err := pubClient.Generate(config.PublisherUrl, accessToken, request)
	if err != nil {
		log.Printf("Error generating html for file %s: %s", path, err)
		return
	}
	log.Printf("Generated html for file %s", path)
	log.Printf("Uploading html for file %s", path)
	url, err := libClient.UploadPost(config.LibrarianUrl, accessToken, libModels.CreateNewPostRequest{
		Post: post,
	})
	if err != nil {
		log.Printf("Error uploading html for file %s: %s", path, err)
		return
	}
	log.Printf("Uploaded html for file %s. The result url: %s", path, url)
}

func doAuthor(paths <-chan string, wg *sync.WaitGroup, config AuthorConfig, accessToken string) {
	if wg != nil {
		defer wg.Done()
	}
	for path := range paths {
		author(path, config, accessToken)
	}
}

func main() {
	config := ConfigFromEnv()

	tokens, err := authClient.Login(config.AuthUrl, authModels.LoginRequest{
		Email:    config.AdminEmail,
		Password: config.AdminPassword,
	})

	if err != nil {
		log.Fatalf("Error retrieving auth token: %s", err)
	}

	inputPaths := os.Args[1:]
	var wg sync.WaitGroup
	wg.Add(MAX_WORKERS)
	paths := make(chan string)

	for i := 0; i < MAX_WORKERS; i++ {
		go doAuthor(paths, &wg, config, tokens.AccessToken)
	}

	for _, path := range inputPaths {
		paths <- path
	}
	close(paths)
	wg.Wait()
}
