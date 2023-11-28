package main

import (
	"context"
	blogModels "github.com/brahms116/pastureen_mono/golang/blog_models"
	models "github.com/brahms116/pastureen_mono/golang/librarian_models"
  "github.com/brahms116/pastureen_mono/golang/librarian/ent"
)

func PostToCreateLinkParams(post *blogModels.Post) models.Link {
	return models.Link{
		Title: post.PostMeta.Title,
		Date:  post.PostMeta.Date,
		Url:   "/posts/" + post.PostMeta.Slug + ".html",
		Tags:  post.PostMeta.Tags,
	}
}

func HandlePost(
	client *ent.Client,
	config *LibrarianConfig,
	post *blogModels.Post,
	ctx context.Context,
) (string, error) {

	errorChan := make(chan error)

	go func() {
		_, err := UploadPostToS3(post.PostHtml, post.PostMeta.Slug, config.BlogBucketName)
		errorChan <- err
	}()

	linkParams := PostToCreateLinkParams(post)
	preparedLink, err := PrepareDbLink(linkParams, client, ctx)
	s3Err := <-errorChan

	if err != nil {
		return "", err
	}

	if s3Err != nil {
		return "", s3Err
	}

	url, err := preparedLink.ID(ctx)

	if err != nil {
		return "", err
	}

	return url, nil
}
