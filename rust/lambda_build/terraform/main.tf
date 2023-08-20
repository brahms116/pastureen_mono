provider "aws" {
  region = "us-east-1"
}

resource "aws_ecrpublic_repository" "rust_lambda_build" {
  repository_name = "rust_lambda_build_container"
}
