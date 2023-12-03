package client

import (
	"bytes"
	"encoding/json"
	"github.com/brahms116/pastureen_mono/golang/http_utils"
	librarianModels "github.com/brahms116/pastureen_mono/golang/librarian_models"
	"io"
	"net/http"
	"net/url"
)

func SearchLinksWithCredentials(credentials librarianModels.Credentials, query librarianModels.QueryLinksRequest) ([]librarianModels.Link, error) {
	return SearchLinks(credentials.Endpoint, query)
}

func SearchLinks(endpoint string, query librarianModels.QueryLinksRequest) ([]librarianModels.Link, error) {
	body, err := json.Marshal(query)
	if err != nil {
		return nil, err
	}
	resp, err := http.Post(endpoint+"/search", "application/json", bytes.NewReader(body))
	var searchResponse librarianModels.QueryLinksResponse
	err = utils.HandleResponse(resp, &searchResponse)
	if err != nil {
		return nil, err
	}
	return searchResponse.Links, err
}

func GetLinkWithCredentials(credentials librarianModels.Credentials, linkUrl string) (*librarianModels.Link, error) {
	return GetLink(credentials.Endpoint, linkUrl)
}

func GetLink(endpoint string, linkUrl string) (*librarianModels.Link, error) {

	resp, err := http.Get(endpoint + "/link?url=" + url.QueryEscape(linkUrl))
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

func UploadPostWithCredentials(credentials librarianModels.Credentials, createPostReq librarianModels.CreateNewPostRequest) (string, error) {
	return UploadPost(credentials.Endpoint, credentials.AccessToken, createPostReq)
}

func UploadPost(
	librarianEndpoint string,
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

	req, err := http.NewRequest("POST", librarianEndpoint+"/post", read)
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
