resource "aws_dynamodb_table" "transaction_types_table" {
  name            = "fin_transaction_types_${var.environment}"
  billing_mode    = "PAY_PER_REQUEST"
  hash_key        = "id"

  attribute {
    name = "id"
    type = "S"
  }
}

resource "aws_dynamodb_table" "classifying_rules_table" {
  name            = "fin_classifying_rules_${var.environment}"
  billing_mode    = "PAY_PER_REQUEST"
  hash_key        = "name"

  attribute {
    name = "name"
    type = "S"
  }
}

resource "aws_dynamodb_table" "unprocessed_transactions_table" {
  name            = "fin_unprocessed_transactions_${var.environment}"
  billing_mode    = "PAY_PER_REQUEST"
  hash_key        = "id"

  attribute {
    name = "id"
    type = "S"
  }
}

resource "aws_dynamodb_table" "transactions_table" {
  name            = "fin_transactions_${var.environment}"
  billing_mode    = "PAY_PER_REQUEST"
  hash_key        = "month"
  range_key       = "date"

  attribute {
    name = "month"
    type = "N"
  }

  attribute {
    name = "date"
    type = "N"
  }
}


