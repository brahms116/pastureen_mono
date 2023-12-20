module "dev" {
  source = "./module"
  environment = "dev"
}

module "test" {
  source = "./module"
  environment = "test"
}

module "prod" {
  source = "./module"
  environment = "prod"
}
