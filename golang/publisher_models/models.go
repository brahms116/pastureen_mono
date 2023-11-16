package models;

import (
  blogModels "github.com/brahms116/pastureen_mono/golang/blog_models"
)

type GeneratePostRequest struct {
	MarkdownString string `json:"markdown"`
}

type GeneratePostResponse struct {
	GeneratedPost blogModels.Post `json:"generatedPost"`
}
