module "fin_deployment_dev" {
  source      = "../fin"
  environment = "dev"
}

module "actix_poc" {
  source = "../actix_poc"
}
