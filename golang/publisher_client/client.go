package client

import (
	"encoding/json"
	authModels "github.com/brahms116/pastureen_mono/golang/auth_models"
	blogModels "github.com/brahms116/pastureen_mono/golang/blog_models"
	"github.com/brahms116/pastureen_mono/golang/http_utils"
	"github.com/brahms116/pastureen_mono/golang/publisher_models"
	"io"
	"net/http"
)

const PUBLISHER_PATH = "/publisher"

func Generate(requestConfig authModels.AuthenticatedApiRequestConfig, generatePostReq models.GeneratePostRequest) (blogModels.Post, error) {

  publisherEndpoint := requestConfig.Endpoint + PUBLISHER_PATH

	read, write := io.Pipe()

	go func() {
		defer write.Close()
		encoder := json.NewEncoder(write)
		encoder.SetEscapeHTML(false)
		encoder.Encode(generatePostReq)
	}()

	request, err := http.NewRequest("POST", publisherEndpoint, read)
	if err != nil {
		return blogModels.Post{}, err
	}
	request.Header.Set("Authorization", "Bearer "+requestConfig.AccessToken)
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
