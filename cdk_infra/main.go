package main

import (
	cdk "github.com/aws/aws-cdk-go/awscdk/v2"
	"github.com/aws/jsii-runtime-go"
)

func main() {
	defer jsii.Close()
	app := cdk.NewApp(nil)
	NewStaticAssetsStack(app, "StaticAssets")
	NewBlogStack(app, "BlogSite")
	app.Synth(nil)
}
