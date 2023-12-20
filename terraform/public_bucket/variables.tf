variable "bucket_name" {
  description = "The name of the S3 bucket"
  type        = string
}

variable "is_website" {
  description = "Whether or not the bucket is a website"
  type        = bool
  default     = false
}

