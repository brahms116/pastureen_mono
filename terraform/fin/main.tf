resource "aws_dynamodb_table" "transaction_types_table" {
  name            = "fin_transaction_types_${var.environment}"
  billing_mode    = "PAY_PER_REQUEST"
  hash_key        = "id"

  attribute {
    name = "id"
    type = "S"
  }
}



