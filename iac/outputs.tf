output "db_endpoint" {
  value = scaleway_sdb_sql_database.todo.endpoint
}

output "db_user" {
  value = scaleway_iam_application.db_app.id
}

output "db_password" {
  value     = scaleway_iam_api_key.db_key.secret_key
  sensitive = true
}

output "db_url" {
  value     = local.db_url_with_options
  sensitive = true
}

output "function_urls" {
  value = { for name, container in scaleway_container.todo : name => container.status == "ready" ? "https://${container.domain_name}" : "pending" }
  description = "The URLs of the serverless functions"
}

output "website_url" {
  value       = "https://${scaleway_object_bucket.todo_ui.name}.s3.${var.region}.scw.cloud"
  description = "The URL of the static website"
}

output "website_endpoint" {
  value       = scaleway_object_bucket_website_configuration.todo_ui.website_endpoint
  description = "The website endpoint of the bucket"
}