package client

import (
	"bytes"
	"encoding/json"
	authModels "github.com/brahms116/pastureen_mono/golang/auth_models"
	"github.com/brahms116/pastureen_mono/golang/http_utils"
	librarianModels "github.com/brahms116/pastureen_mono/golang/librarian_models"
	"io"
	"net/http"
	"net/url"
)

const LIBRARIAN_PATH = "/librarian"

func SearchLinks(endpoint string, query librarianModels.QueryLinksRequest) ([]librarianModels.Link, error) {

	librarianEndpoint := endpoint + LIBRARIAN_PATH

	body, err := json.Marshal(query)
	if err != nil {
		return nil, err
	}
	resp, err := http.Post(librarianEndpoint+"/search", "application/json", bytes.NewReader(body))
	var searchResponse librarianModels.QueryLinksResponse
	err = utils.HandleResponse(resp, &searchResponse)
	if err != nil {
		return nil, err
	}
	return searchResponse.Links, err
}

func GetLink(endpoint string, linkUrl string) (*librarianModels.Link, error) {
	librarianEndpoint := endpoint + LIBRARIAN_PATH

	resp, err := http.Get(librarianEndpoint + "/link?url=" + url.QueryEscape(linkUrl))
	if err != nil {
		return nil, err
	}
	var getLinkResponse librarianModels.GetLinkResponse
	err = utils.HandleResponse(resp, &getLinkResponse)
	if err != nil {
		return nil, err
	}
	return getLinkResponse.Link, err
}

func UploadPost(
	requestConfig authModels.AuthenticatedApiRequestConfig,
	createPostReq librarianModels.CreateNewPostRequest,
) (string, error) {
	librarianEndpoint := requestConfig.Endpoint + LIBRARIAN_PATH

	read, write := io.Pipe()

	go func() {
		defer write.Close()
		encoder := json.NewEncoder(write)
		encoder.SetEscapeHTML(false)
		encoder.Encode(createPostReq)
	}()

	req, err := http.NewRequest("POST", librarianEndpoint+"/post", read)
	if err != nil {
		return "", err
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+requestConfig.AccessToken)
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
