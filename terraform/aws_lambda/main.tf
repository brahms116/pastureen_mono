resource "aws_lambda_function" "lambda" {
  function_name    = var.lambda_function_name
  role             = aws_iam_role.lambda_role.arn
  architectures    = ["x86_64"]
  package_type     = "Zip"
  handler          = "bootstrap"
  runtime          = "provided.al2"
  source_code_hash = filebase64sha256("${path.module}/dummy.zip")
  filename         = "${path.module}/dummy.zip"
  layers           = local.lambda_layers
  environment {
    variables = var.lambda_environment_variables
  }

  lifecycle {
    ignore_changes = [
      source_code_hash,
      filename,
      environment
    ]
  }
}


resource "aws_iam_role" "lambda_role" {
  name = "${var.lambda_function_name}_role"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
        Sid = ""
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "lambda_role_policy" {
  role       = aws_iam_role.lambda_role.name
  policy_arn = var.lambda_execution_role_policy
}
