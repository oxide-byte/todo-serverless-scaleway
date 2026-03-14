provider "scaleway" {
  zone   = "fr-par-1"
  region = "fr-par"
  access_key = var.access_key
  secret_key = var.secret_key
  project_id = var.project_id
}

provider "postgresql" {
  host     = split("/", split("@", scaleway_sdb_sql_database.todo.endpoint)[1])[0]
  port     = 5432
  database = var.db_name
  username = var.access_key
  password = var.secret_key
  sslmode  = "require"
  connect_timeout = 15
}