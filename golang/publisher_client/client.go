package client

import (
	"bytes"
	"net/http"
	"pastureen/publisher-models"
	"pastureen/http-utils"
  "encoding/json"
)

func Publish(endpoint string, accessToken string, generatePostReq models.GeneratePostRequest) (models.Post, error) {
  body, err := json.Marshal(generatePostReq)
  if err != nil {
    return models.Post{}, err
  }

	request, err := http.NewRequest("POST", endpoint, bytes.NewBuffer(body))
	if err != nil {
		return models.Post{}, err
	}
	request.Header.Set("Authorization", "Bearer "+accessToken)

	response, err := http.DefaultClient.Do(request)
	if err != nil {
		return models.Post{}, err
	}

	var postResponse models.GeneratePostResponse
  err = utils.HandleResponse(response, &postResponse)
	return postResponse.GeneratedPost, err
}
