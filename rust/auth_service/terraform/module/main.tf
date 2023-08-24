
module "function" {
  source               = "../../../terraform/aws_lambda"
  lambda_function_name = "${local.service}_${var.environment}"
  http_adapter         = true
}

resource "aws_lambda_function_url" "lambda_url" {
  function_name      = "${local.service}_${var.environment}"
  authorization_type = "NONE"

  depends_on = [
    module.function
  ]

  cors {
    allow_credentials = true
    allow_origins     = ["*"]
    allow_methods     = ["*"]
    allow_headers     = ["*"]
    expose_headers    = ["*"]
    max_age           = 86400
  }
}
