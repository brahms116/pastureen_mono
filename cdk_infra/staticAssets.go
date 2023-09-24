package main

import (
	cdk "github.com/aws/aws-cdk-go/awscdk/v2"
	s3 "github.com/aws/aws-cdk-go/awscdk/v2/awss3"
	"github.com/aws/constructs-go/constructs/v10"
	"github.com/aws/jsii-runtime-go"
)

type AssetsBucketProps struct {
	Env string
}

func NewAssetsBucket(scope constructs.Construct, id string, props AssetsBucketProps) s3.Bucket {
  env := parseEnv(props.Env)
  sprops := getPublicBucketProps(env)
  sprops.BucketName = jsii.String("pastureen-static-assets-" + env)
	return s3.NewBucket(scope, &id, &sprops)
}

func NewStaticAssetsStack(scope constructs.Construct, id string) cdk.Stack {
	stack := cdk.NewStack(scope, &id, nil)
	NewAssetsBucket(stack, "AssetsBucketDev", AssetsBucketProps{Env: ENV_DEV})
	NewAssetsBucket(stack, "AssetsBucketProd", AssetsBucketProps{Env: ENV_PROD})
	NewAssetsBucket(stack, "AssetsBucketStaging", AssetsBucketProps{Env: ENV_TEST})
	return stack
}
