module "lambda" {
  source               = "../aws_lambda"
  lambda_function_name = "router_service_${var.environment}"
  zip_location         = "../../build/rust_lambda/router_service.zip"
}
