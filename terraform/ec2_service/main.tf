
module "lambda" {
  source               = "../rust_aws_lambda"
  lambda_function_name = "ec2_service_${var.environment}"
  lambda_zip_path      = "../../rust/build_outputs/ec2_service/bootstrap.zip"
}

