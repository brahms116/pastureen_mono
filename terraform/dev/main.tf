module "router_service_development" {
  source      = "../router_service"
  environment = "dev"
  is_initial_deployment = var.is_initial_deployment
}
