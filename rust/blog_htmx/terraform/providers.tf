provider "aws" {
  region = "ap-southeast-2"
}

terraform {
  backend "s3" {
    key    = "pastureen/blog_htmx.tfstate"
    bucket = "pastureen-tf-state-store"
    region = "ap-southeast-2"
  }
}

