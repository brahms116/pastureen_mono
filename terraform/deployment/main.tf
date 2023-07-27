module "router_service_development_dev" {
  source      = "../router_service"
  environment = "dev"
}

module "fin_deployment_dev" {
  source      = "../fin"
  environment = "dev"
}
