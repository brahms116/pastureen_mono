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

	env := props.Env
	if env == "" {
		env = "dev"
	}

	removalPolicy := cdk.RemovalPolicy_DESTROY

	if env == "prod" {
		removalPolicy = cdk.RemovalPolicy_RETAIN
	}

	sprops := s3.BucketProps{
		BucketName:        jsii.String("pastureen-static-assets-" + props.Env),
		BlockPublicAccess: s3.BlockPublicAccess_BLOCK_ACLS(),
		ObjectOwnership:   s3.ObjectOwnership_BUCKET_OWNER_ENFORCED,
		PublicReadAccess:  jsii.Bool(true),
		RemovalPolicy:     removalPolicy,
	}

	return s3.NewBucket(scope, &id, &sprops)
}

func NewStaticAssetsStack(scope constructs.Construct, id string) cdk.Stack {
	stack := cdk.NewStack(scope, &id, nil)
	NewAssetsBucket(stack, "AssetsBucketDev", AssetsBucketProps{Env: "dev"})
	NewAssetsBucket(stack, "AssetsBucketProd", AssetsBucketProps{Env: "prod"})
	NewAssetsBucket(stack, "AssetsBucketStaging", AssetsBucketProps{Env: "test"})
	return stack
}
