locals {
  lambda_layers = var.http_adapter ? ["arn:aws:lambda:ap-southeast-2:753240598075:layer:LambdaAdapterLayerArm64:16"] : []
}
