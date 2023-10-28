module "function" {
  source                       = "../../../../terraform/aws_lambda"
  lambda_function_name         = "${local.service}_${var.environment}"
  http_adapter                 = true
  lambda_execution_role_policy = aws_iam_policy.librarian_policy.arn
}

resource "aws_iam_role_policy" "librarian_policy" {
  name = "librarian_policy"
  role = module.function.lambda_role_name

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = [
          "s3:*",
        ]
        Effect = "Allow"
        Resource = [
          "arn:aws:s3:::pastureen-blog-${var.environment}",
          "arn:aws:s3:::pastureen-blog-${var.environment}/*",
        ]
      },
      {
        Action = [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents"
        ]
        Effect = "Allow"
        Resource = "*"
      }
    ]
  })

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
