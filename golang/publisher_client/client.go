package client

import (
	"encoding/json"
	authClient "github.com/brahms116/pastureen_mono/golang/auth_client"
	blogModels "github.com/brahms116/pastureen_mono/golang/blog_models"
	"github.com/brahms116/pastureen_mono/golang/http_utils"
	"github.com/brahms116/pastureen_mono/golang/publisher_models"
	"io"
	"net/http"
)

type AccessCredentials struct {
	AccessToken       string
	PublisherEndpoint string
}

func NewAccessCredentials(accessToken string, publisherEndpoint string) AccessCredentials {
	return AccessCredentials{
		AccessToken:       accessToken,
		PublisherEndpoint: publisherEndpoint,
	}
}

func (c AccessCredentials) GeneratePost(generatePostReq models.GeneratePostRequest) (blogModels.Post, error) {
	return generate(c, generatePostReq)
}

type PublisherClientConfig struct {
	PublisherEndpoint string
	TokenCredentials  authClient.TokenCredentials
}

func NewPublisherClientConfig(publisherEndpoint string, tokenCredentials authClient.TokenCredentials) PublisherClientConfig {
	return PublisherClientConfig{
		PublisherEndpoint: publisherEndpoint,
		TokenCredentials:  tokenCredentials,
	}
}

func (c PublisherClientConfig) ToAccessCredentials() AccessCredentials {
	return NewAccessCredentials(
		c.TokenCredentials.AccessToken, c.PublisherEndpoint,
	)
}

func (c PublisherClientConfig) GeneratePost(generatePostReq models.GeneratePostRequest) (blogModels.Post, error) {
	return c.ToAccessCredentials().GeneratePost(generatePostReq)
}

func generate(accessCredentials AccessCredentials, generatePostReq models.GeneratePostRequest) (blogModels.Post, error) {

	read, write := io.Pipe()

	go func() {
		defer write.Close()
		encoder := json.NewEncoder(write)
		encoder.SetEscapeHTML(false)
		encoder.Encode(generatePostReq)
	}()

	request, err := http.NewRequest("POST", accessCredentials.PublisherEndpoint, read)
	if err != nil {
		return blogModels.Post{}, err
	}
	request.Header.Set("Authorization", "Bearer "+accessCredentials.AccessToken)
	request.Header.Set("Content-Type", "application/json")

	response, err := http.DefaultClient.Do(request)
	if err != nil {
		return blogModels.Post{}, err
	}

	var postResponse models.GeneratePostResponse
	err = utils.HandleResponse(response, &postResponse)
	if err != nil {
		return blogModels.Post{}, err
	}
	return postResponse.GeneratedPost, err
}
