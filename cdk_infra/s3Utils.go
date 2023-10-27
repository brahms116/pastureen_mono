package main

import (
	cdk "github.com/aws/aws-cdk-go/awscdk/v2"
	s3 "github.com/aws/aws-cdk-go/awscdk/v2/awss3"
	"github.com/aws/jsii-runtime-go"
)

func getPublicBucketProps(env string) s3.BucketProps {
	removalPolicy := cdk.RemovalPolicy_DESTROY

	if env == "prod" {
		removalPolicy = cdk.RemovalPolicy_RETAIN
	}

	return s3.BucketProps{
		BlockPublicAccess: s3.BlockPublicAccess_BLOCK_ACLS(),
		ObjectOwnership:   s3.ObjectOwnership_BUCKET_OWNER_ENFORCED,
		PublicReadAccess:  jsii.Bool(true),
		RemovalPolicy:     removalPolicy,
	}
}
