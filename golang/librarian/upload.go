package main

import (
	"strings"
	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/s3/s3manager"
)

func UploadPostToS3(html string, slug string, bucket string) (string, error) {
	sess := session.Must(session.NewSession())
	uploader := s3manager.NewUploader(sess)

	uploadResult, err := uploader.Upload(&s3manager.UploadInput{
		Bucket: aws.String(bucket),
		Key:    aws.String("posts/" + slug + ".html"),
		Body:   strings.NewReader(html),
	})

	if err != nil {
		return "", err
	}

	return uploadResult.Location, nil
}
