module "lambda" {
  source               = "../aws_lambda"
  lambda_function_name = "router_service_${var.environment}"
  zip_location         = "../dummy.zip"
}
