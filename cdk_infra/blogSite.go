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

func NewBlogBucket (scope constructs.Construct, id string, props BlogBucketProps) s3.Bucket {
  env := props.Env
  if env == "" {
    env = "dev"
  }

  sprops := s3.BucketProps{
    BucketName: jsii.String("pastureen-blog-site-" + props.Env),
    BlockPublicAccess: s3.BlockPublicAccess_BLOCK_ACLS(),
    ObjectOwnership: s3.ObjectOwnership_BUCKET_OWNER_ENFORCED,
    PublicReadAccess: jsii.Bool(true),
  }

  return s3.NewBucket(scope, &id, &sprops)
}


func NewBlogStack(scope constructs.Construct, id string) cdk.Stack {
  stack := cdk.NewStack(scope, &id, nil)
  NewBlogBucket(stack, "BlogBucket", BlogBucketProps{Env: "dev"})
  NewBlogBucket(stack, "BlogBucketProd", BlogBucketProps{Env: "prod"})
  NewBlogBucket(stack, "BlogBucketStaging", BlogBucketProps{Env: "test"})

  return stack
}
