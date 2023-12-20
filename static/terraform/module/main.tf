module "bucket" {
  source = "../../../terraform/public_bucket"
  bucket_name = "pastureen-static-assets-${var.environment}"
}

