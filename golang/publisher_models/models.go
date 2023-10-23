package models;

import (
  blogModels "pastureen/blog-models"
)

type GeneratePostRequest struct {
	MarkdownString string `json:"markdown"`
}

type GeneratePostResponse struct {
	GeneratedPost blogModels.Post `json:"generatedPost"`
}
