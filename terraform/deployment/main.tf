module "router_service_development" {
  source      = "../router_service"
  environment = "dev"
}

module "fin_deployment" {
  source      = "../fin"
  environment = "dev"
}
