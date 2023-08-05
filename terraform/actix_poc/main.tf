module "lambda" {
  source               = "../aws_lambda"
  lambda_function_name = "actix_poc_dev"
  zip_location         = "../dummy.zip"
  http_adapter         = true
}

resource "aws_lambda_function_url" "lambda_url" {
  function_name      = "actix_poc_dev"
  authorization_type = "NONE"

  depends_on = [
    module.lambda
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



