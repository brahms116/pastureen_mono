data "aws_ecr_repository" "repo" {
  name = "router_service"
}

module "lambda" {
  source               = "../aws_lambda"
  lambda_function_name = "router_service_${var.environment}"
  ecr_image_uri            = "${data.aws_ecr_repository.repo.repository_url}:${var.environment}"
}
