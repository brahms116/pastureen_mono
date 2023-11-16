package models

import (
  blogModels "github.com/brahms116/pastureen_mono/golang/blog_models"
)

type Link struct {
	Title       string   `json:"title"`
	Date        string   `json:"date"`
	Url         string   `json:"url"`
	Subtitle    string   `json:"subtitle"`
	Description string   `json:"description"`
	ImageUrl    string   `json:"imageUrl,omitempty"`
	ImageAlt    string   `json:"imageAlt,omitempty"`
	Tags        []string `json:"tags"`
}

type PaginationRequest struct {
	Page  int `json:"page"`
	Limit int `json:"limit"`
}

type QueryLinksRequest struct {
	Pagination PaginationRequest `json:"pagination"`
	Tags       []string `json:"tags"`
	TitleQuery string   `json:"titleQuery"`
	StartDate  string   `json:"startDate"`
	EndDate    string   `json:"endDate"`
}

type QueryLinksResponse struct {
	Links []Link `json:"links"`
}

type GetLinkRequest struct {
  Url string `form:"url"`
}

type GetLinkResponse struct {
  // Has to be a pointer to allow for null values
  Link *Link `json:"link"`
}

type CreateNewPostRequest struct {
  Post blogModels.Post `json:"post"`
}

type CreateNewPostResponse struct {
  Url string `json:"url"`
}

type GetTagsResponse struct {
  Tags []string `json:"tags"`
}
