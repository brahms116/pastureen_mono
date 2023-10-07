package client

import (
	"net/http"
	"pastureen/publisher-models"
	"pastureen/http-utils"
  "encoding/json"
  "io"
)

func Generate(endpoint string, accessToken string, generatePostReq models.GeneratePostRequest) (models.Post, error) {

  read, write := io.Pipe()

  go func() {
    defer write.Close()
    encoder := json.NewEncoder(write)
    encoder.SetEscapeHTML(false)
    encoder.Encode(generatePostReq)
  }()

	request, err := http.NewRequest("POST", endpoint, read)
	if err != nil {
		return models.Post{}, err
	}
	request.Header.Set("Authorization", "Bearer "+accessToken)
  request.Header.Set("Content-Type", "application/json")

	response, err := http.DefaultClient.Do(request)
	if err != nil {
		return models.Post{}, err
	}

	var postResponse models.GeneratePostResponse
  err = utils.HandleResponse(response, &postResponse)
	return postResponse.GeneratedPost, err
}
