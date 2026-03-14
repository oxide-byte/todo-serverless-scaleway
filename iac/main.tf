resource "scaleway_container_namespace" "main" {
  name        = "serverless-todo-ns"
  description = "Namespace managed by terraform"
}

resource "scaleway_registry_namespace" "todo" {
  name        = "todo-registry"
  description = "Registry for todo container images"
}

locals {
  functions = [
    { name = "add", dockerfile = "build-add.dockerfile", binary = "add-todo" },
    { name = "delete", dockerfile = "build-delete.dockerfile", binary = "delete-todo" },
    { name = "get", dockerfile = "build-get.dockerfile", binary = "get-todo" },
    { name = "get-all", dockerfile = "build-get-all.dockerfile", binary = "get-all-todo" },
    { name = "edit", dockerfile = "build-edit.dockerfile", binary = "edit-todo" }
  ]
  db_host = split(":", split("/", replace(scaleway_sdb_sql_database.todo.endpoint, "postgres://", ""))[0])[0]
  db_url = "postgres://${urlencode(var.access_key)}:${urlencode(var.secret_key)}@${local.db_host}/${var.db_name}?sslmode=require"
}

resource "null_resource" "docker_build_push" {
  for_each = { for f in local.functions : f.name => f }

  triggers = {
    dockerfile_hash = filemd5("${path.module}/../${each.value.dockerfile}")
  }

  provisioner "local-exec" {
    command = <<EOT
      docker build -t ${scaleway_registry_namespace.todo.endpoint}/${each.value.name}:latest -f ../${each.value.dockerfile} ..
      docker login ${scaleway_registry_namespace.todo.endpoint} -u nologin -p ${var.secret_key}
      docker push ${scaleway_registry_namespace.todo.endpoint}/${each.value.name}:latest
    EOT
  }
}

resource "scaleway_container" "todo" {
  for_each     = { for f in local.functions : f.name => f }
  name         = "todo-${each.value.name}"
  namespace_id = scaleway_container_namespace.main.id
  registry_image = "${scaleway_registry_namespace.todo.endpoint}/${each.value.name}:latest"
  port         = 8080
  cpu_limit    = 140
  memory_limit = 256
  min_scale    = 0
  max_scale    = 5
  timeout      = 60

  environment_variables = {
    DATABASE_URL = local.db_url
  }

  depends_on = [null_resource.docker_build_push]
}

resource "scaleway_sdb_sql_database" "todo" {
  name    = var.db_name
  max_cpu = 15
}

data "local_file" "init_sql" {
  filename = "${path.module}/../infrastructure/postgres/init.sql"
}

resource "null_resource" "db_setup" {
  triggers = {
    file_hash = data.local_file.init_sql.content_md5
    db_id     = scaleway_sdb_sql_database.todo.id
  }

  # Increase retries for IAM propagation
  provisioner "local-exec" {
    environment = {
      PGPASSWORD = var.secret_key
      PGUSER     = var.access_key
      PGHOST     = local.db_host
      PGDATABASE = var.db_name
      PGPORT     = "5432"
      PGSSLMODE  = "require"
    }
    command = <<EOT
      # No set -e at top to allow retry
      for i in {1..20}; do
        echo "Attempt $i: Connecting to $PGHOST as $PGUSER..."
        if psql -f "${data.local_file.init_sql.filename}" 2>&1; then
          exit 0
        fi
        echo "Database setup failed (attempt $i/20), retrying in 30 seconds..."
        sleep 30
      done
      exit 1
    EOT
  }
}