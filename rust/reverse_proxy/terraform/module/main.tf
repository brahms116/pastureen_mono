
provider "aws" {
  alias  = "aws_us_east_1"
  region = "us-east-1"
}

data "aws_acm_certificate" "cert" {
  domain   = "*.pastureen.davidkwong.net"
  provider = aws.aws_us_east_1
}

data "aws_route53_zone" "zone" {
  name = "pastureen.davidkwong.net"
}

module "function" {
  source               = "../../../../terraform/aws_lambda"
  lambda_function_name = "${local.service}_${var.environment}"
  http_adapter         = true
}

resource "aws_lambda_function_url" "lambda_url" {
  function_name      = "${local.service}_${var.environment}"
  authorization_type = "NONE"

  depends_on = [
    module.function
  ]

  cors {
    allow_credentials = true
    allow_origins     = ["*"]
    allow_methods     = ["*"]
    allow_headers     = ["*"]
    expose_headers    = ["*"]
    max_age           = 86400
  }
}

resource "aws_cloudfront_distribution" "distrubution" {
  default_cache_behavior {
    allowed_methods        = ["GET", "HEAD", "OPTIONS", "PUT", "POST", "PATCH", "DELETE"]
    cache_policy_id        = "4135ea2d-6df8-44a3-9df3-4b5a84be39ad"
    cached_methods         = ["HEAD", "GET"]
    target_origin_id       = "lambda"
    viewer_protocol_policy = "allow-all"
  }

  enabled = true


  aliases = ["${local.path_prefix}pastureen.davidkwong.net"]

  origin {
    domain_name = substr(aws_lambda_function_url.lambda_url.function_url, 8, length(aws_lambda_function_url.lambda_url.function_url) - 9)
    origin_id   = "lambda"

    custom_origin_config {
      http_port              = 80
      https_port             = 443
      origin_protocol_policy = "https-only"
      origin_ssl_protocols   = ["TLSv1.2"]
    }
  }

  viewer_certificate {
    acm_certificate_arn = data.aws_acm_certificate.cert.arn
    ssl_support_method  = "sni-only"
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }
}

resource "aws_route53_record" "record" {
  zone_id = data.aws_route53_zone.zone.zone_id
  name    = "${local.path_prefix}pastureen.davidkwong.net"
  type    = "A"

  alias {
    name                   = aws_cloudfront_distribution.distrubution.domain_name
    zone_id                = aws_cloudfront_distribution.distrubution.hosted_zone_id
    evaluate_target_health = false
  }

}
