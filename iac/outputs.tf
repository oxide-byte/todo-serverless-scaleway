output "db_endpoint" {
  value = scaleway_sdb_sql_database.todo.endpoint
}

output "db_user" {
  value = var.access_key
}

output "db_password" {
  value     = var.secret_key
  sensitive = true
}

output "db_url" {
  value     = local.db_url
  sensitive = true
}

output "function_urls" {
  value = { for name, container in scaleway_container.todo : name => container.status == "ready" ? "https://${container.domain_name}" : "pending" }
  description = "The URLs of the serverless functions"
}