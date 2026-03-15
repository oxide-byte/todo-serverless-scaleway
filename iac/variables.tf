variable "access_key" {
  type = string
}

variable "secret_key" {
  type = string
}

variable "project_id" {
  type = string
}

variable "organization_id" {
  type = string
}

variable "region" {
  type    = string
  default = "fr-par"
}

variable "db_name" {
  type    = string
  default = "todo-db"
}