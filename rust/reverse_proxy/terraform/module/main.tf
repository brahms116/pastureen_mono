
module "function" {
  source               = "../../../terraform/aws_lambda"
  lambda_function_name = "reverse_proxy_${var.environment}"
  http_adapter         = true
}

resource "aws_lambda_function_url" "lambda_url" {
  function_name      = "reverse_proxy_${var.environment}"
  authorization_type = "NONE"

  depends_on = [
    module.function
  ]

  cors {
    allow_credentials = true
    allow_origins     = ["*"]
    allow_methods     = ["*"]
    allow_headers     = ["date", "keep-alive"]
    expose_headers    = ["keep-alive", "date"]
    max_age           = 86400
  }
}
