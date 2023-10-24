package main

import (
	"context"
	blogModels "pastureen/blog-models"
	models "pastureen/librarian-models"
	"pastureen/librarian/ent"
)

func PostToCreateLinkParams(post *blogModels.Post, postsUrl string) models.CreateLinkParams {
	return models.CreateLinkParams{
		Title: post.PostMeta.Title,
		Date:  post.PostMeta.Date,
		Url:   postsUrl + "/" + post.PostMeta.Slug,
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

	linkParams := PostToCreateLinkParams(post, config.BlogUrl+"/posts")
	preparedLink, err := PrepareDbLink(linkParams, client, ctx)
	s3Err := <-errorChan

	if err != nil || s3Err != nil {
		return "", err
	}

	createdDbLink, err := preparedLink.Save(ctx)

	if err != nil {
		return "", err
	}

	return createdDbLink.URL, nil
}
