resource "aws_s3_bucket" "my_bucket" {
  bucket = var.bucket_name
}

resource "aws_s3_bucket_public_access_block" "block" {
  bucket = aws_s3_bucket.my_bucket.id
  block_public_acls       = true
}

resource "aws_s3_bucket_policy" "policy" {
  bucket = aws_s3_bucket.my_bucket.id
  policy = <<POLICY
  {
    "Version": "2012-10-17",
    "Statement": [
      {
        "Effect": "Allow",
        "Principal": "*",
        "Action": [
          "s3:GetObject"
        ],
        "Resource": [
          "arn:aws:s3:::${var.bucket_name}/*"
        ]
      }
    ]
  }
  POLICY
}

resource "aws_s3_bucket_website_configuration" "website_config" {
  count = var.is_website ? 1 : 0
  bucket = aws_s3_bucket.my_bucket.id
  index_document {
    suffix = "index.html"
  }
}
