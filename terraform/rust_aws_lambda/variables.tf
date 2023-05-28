variable "lambda_function_name" {
  type        = string
  description = "Name of the lambda function"
}

variable "lambda_zip_path" {
  type        = string
  description = "Path to the lambda built zip file"
}

variable "lambda_execution_role_policy" {
  type        = string
  description = "Policy to attach to the lambda execution role"
  default     = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

variable "lambda_environment_variables" {
  type        = map(string)
  description = "Environment variables to pass to the lambda function"
  default     = {}
}
