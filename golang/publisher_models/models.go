package models;


type GeneratePostRequest struct {
	MarkdownString string `json:"markdown"`
}

type PostMeta struct {
	Title string   `json:"title"`
	Slug  string   `json:"slug"`
	Tags  []string `json:"tags"`
	Date  string   `json:"date"`
}

type Post struct {
	PostMeta PostMeta `json:"meta"`
	PostHtml string   `json:"postHtml"`
}

type GeneratePostResponse struct {
	GeneratedPost Post `json:"generatedPost"`
}
