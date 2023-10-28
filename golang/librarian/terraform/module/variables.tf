variable "environment" {
  description = "The environment to deploy to"
  validation {
    condition     = var.environment == "dev" || var.environment == "prod" || var.environment == "test"
    error_message = "Environment must be one of dev, prod, or test"
  }
}

