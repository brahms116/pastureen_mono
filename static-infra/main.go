package main

import (
	cdk "github.com/aws/aws-cdk-go/awscdk/v2"
  s3 "github.com/aws/aws-cdk-go/awscdk/v2/awss3"
	"github.com/aws/constructs-go/constructs/v10"
	"github.com/aws/jsii-runtime-go"
)

type StaticAssetsStackProps struct {
	cdk.StackProps
}


type AssetsBucketProps struct {
  Env string
}

func NewAssetsBucket(scope constructs.Construct, id string, props AssetsBucketProps) s3.Bucket {

  env := props.Env
  if env == "" {
    env = "dev"
  }

  sprops := s3.BucketProps{
    BucketName: jsii.String("pastureen-static-assets-"+props.Env),
    BlockPublicAccess: s3.BlockPublicAccess_BLOCK_ACLS(),
    ObjectOwnership: s3.ObjectOwnership_BUCKET_OWNER_ENFORCED,
    PublicReadAccess: jsii.Bool(true),
  }

  return s3.NewBucket(scope, &id, &sprops)
}

func NewInfraStack(scope constructs.Construct, id string, props *StaticAssetsStackProps) cdk.Stack {
	var sprops cdk.StackProps
	if props != nil {
		sprops = props.StackProps
	}
	stack := cdk.NewStack(scope, &id, &sprops)

  NewAssetsBucket(stack, "AssetsBucket", AssetsBucketProps{Env: "dev"})
  NewAssetsBucket(stack, "AssetsBucketProd", AssetsBucketProps{Env: "prod"})
  NewAssetsBucket(stack, "AssetsBucketStaging", AssetsBucketProps{Env: "test"})

	return stack
}

func main() {
	defer jsii.Close()

	app := cdk.NewApp(nil)

	NewInfraStack(app, "StaticAssets", &StaticAssetsStackProps{})

	app.Synth(nil)
}

func env() *cdk.Environment {
	return nil
}
