
variable "repository_name" {
  type        = string
  description = "The name of the repository to create"
}

resource "aws_ecr_repository" "ecr" {
  name = var.repository_name
}

output "repository_url" {
  value =  aws_ecr_repository.ecr.repository_url
}
