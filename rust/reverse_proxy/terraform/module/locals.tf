
locals {
  service = "reverse_proxy"
  path_prefix = var.environment == "prod" ? "" : "${var.environment}."
}
