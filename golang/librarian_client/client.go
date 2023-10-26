package client

import (
	"encoding/json"
	"io"
	"net/http"
	librarianModels "pastureen/librarian-models"
  "pastureen/http-utils"
)

func UploadPost(
	endpoint string,
  accessToken string,
	createPostReq librarianModels.CreateNewPostRequest,
) (string, error) {
  read, write := io.Pipe()

  go func() {
    defer write.Close()
    encoder := json.NewEncoder(write)
    encoder.SetEscapeHTML(false)
    encoder.Encode(createPostReq)
  }()

  req, err := http.NewRequest("POST", endpoint+"/post", read)
  if err != nil {
    return "", err
  }
  req.Header.Set("Content-Type", "application/json")
  req.Header.Set("Authorization", "Bearer "+accessToken)
  resp, err := http.DefaultClient.Do(req)
  if err != nil {
    return "", err
  }

  var uploadResponse librarianModels.CreateNewPostResponse
  err = utils.HandleResponse(resp, &uploadResponse)
  if err != nil {
    return "", err
  }
  return uploadResponse.Url, err
}
