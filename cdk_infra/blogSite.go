package main

import (
	cdk "github.com/aws/aws-cdk-go/awscdk/v2"
	s3 "github.com/aws/aws-cdk-go/awscdk/v2/awss3"
	"github.com/aws/constructs-go/constructs/v10"
	"github.com/aws/jsii-runtime-go"
)

type BlogBucketProps struct {
	Env string
}

func NewBlogBucket(scope constructs.Construct, id string, props BlogBucketProps) s3.Bucket {
	env := parseEnv(props.Env)
	sprops := getPublicBucketProps(env)
	sprops.BucketName = jsii.String("pastureen-blog-" + env)
	sprops.WebsiteIndexDocument = jsii.String("index.html")

  // Delete objects after 1 day in test environment
  if env == ENV_TEST {
    sprops.LifecycleRules = &[]*s3.LifecycleRule{
      {
        Enabled: jsii.Bool(true),
        Prefix: jsii.String("posts/"),
        Expiration: cdk.Duration_Days(jsii.Number(1)),
      },
    }

  }

	return s3.NewBucket(scope, &id, &sprops)
}

func NewBlogStack(scope constructs.Construct, id string) cdk.Stack {
	stack := cdk.NewStack(scope, &id, nil)
	NewBlogBucket(stack, "BlogBucketDev", BlogBucketProps{Env: ENV_DEV})
	NewBlogBucket(stack, "BlogBucketProd", BlogBucketProps{Env: ENV_PROD})
	NewBlogBucket(stack, "BlogBucketStaging", BlogBucketProps{Env: ENV_TEST})

	return stack
}
