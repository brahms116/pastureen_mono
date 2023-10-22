package models

type Link struct {
	Id          string   `json:"id"`
	Title       string   `json:"title"`
	Date        string   `json:"date"`
	Url         string   `json:"url"`
	Subtitle    string   `json:"subtitle"`
	Description string   `json:"description"`
	ImageUrl    string   `json:"imageUrl"`
	ImageAlt    string   `json:"imageAlt"`
	Tags        []string `json:"tags"`
}

type PaginationRequest struct {
	Page  int `json:"page"`
	Limit int `json:"limit"`
}

type QueryLinksRequest struct {
	PaginationRequest
	Tags       []string `json:"tags"`
	TitleQuery string   `json:"titleQuery"`
	StartDate  string   `json:"startDate"`
	EndDate    string   `json:"endDate"`
}

type QueryLinksResponse struct {
	Links []Link `json:"links"`
}

type CreateNewPostRequest struct {
}
