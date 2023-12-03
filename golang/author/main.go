package main

import (
	libClient "github.com/brahms116/pastureen_mono/golang/librarian_client"
	libModels "github.com/brahms116/pastureen_mono/golang/librarian_models"
	pastureen "github.com/brahms116/pastureen_mono/golang/pastureen_client"
	pubClient "github.com/brahms116/pastureen_mono/golang/publisher_client"
	pubModels "github.com/brahms116/pastureen_mono/golang/publisher_models"
	"log"
	"os"
	"sync"
)

const MAX_WORKERS = 10

func author(path string, tokenCreds pastureen.TokenCredentials) {
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
	post, err := pubClient.GeneratePostWithCredentials(
		pastureen.ToPublisherCredentials(tokenCreds), request,
	)
	if err != nil {
		log.Printf("Error generating html for file %s: %s", path, err)
		return
	}

	log.Printf("Generated html for file %s", path)
	log.Printf("Uploading html for file %s", path)
	url, err := libClient.UploadPostWithCredentials(pastureen.ToLibrarianCredentials(tokenCreds), libModels.CreateNewPostRequest{
		Post: post,
	})
	if err != nil {
		log.Printf("Error uploading html for file %s: %s", path, err)
		return
	}
	log.Printf("Uploaded html for file %s. The result url: %s", path, url)
}

func doAuthor(paths <-chan string, wg *sync.WaitGroup, tokenCreds pastureen.TokenCredentials) {
	if wg != nil {
		defer wg.Done()
	}
	for path := range paths {
		author(path, tokenCreds)
	}
}

func main() {

	credentials, err := pastureen.ResolveCredentials()

	if err != nil {
		log.Fatalf("Error resolving credentials: %s", err)
	}

	tokenCreds, err := pastureen.Login(credentials)

	if err != nil {
		log.Fatalf("Error retrieving auth token: %s", err)
	}

	inputPaths := os.Args[1:]
	var wg sync.WaitGroup
	wg.Add(MAX_WORKERS)
	paths := make(chan string)

	for i := 0; i < MAX_WORKERS; i++ {
		go doAuthor(paths, &wg, tokenCreds)
	}

	for _, path := range inputPaths {
		paths <- path
	}
	close(paths)
	wg.Wait()
}
