module "bucket" {
  source = "../../../../terraform/public_bucket"
  bucket_name = "pastureen-blog-${var.environment}"
  is_website = true
}

