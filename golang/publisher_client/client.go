package client

import (
	"encoding/json"
	blogModels "github.com/brahms116/pastureen_mono/golang/blog_models"
	"github.com/brahms116/pastureen_mono/golang/http_utils"
	"github.com/brahms116/pastureen_mono/golang/publisher_models"
	"io"
	"net/http"
)

func GeneratePostWithCredentials(
	credentials models.Credentials,
	generatePostReq models.GeneratePostRequest,
) (blogModels.Post, error) {
	return GeneratePost(credentials.Endpoint, credentials.AccessToken, generatePostReq)
}

func GeneratePost(endpoint string, accessToken string, generatePostReq models.GeneratePostRequest) (blogModels.Post, error) {

	read, write := io.Pipe()

	go func() {
		defer write.Close()
		encoder := json.NewEncoder(write)
		encoder.SetEscapeHTML(false)
		encoder.Encode(generatePostReq)
	}()

	request, err := http.NewRequest("POST", endpoint, read)
	if err != nil {
		return blogModels.Post{}, err
	}
	request.Header.Set("Authorization", "Bearer "+accessToken)
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
