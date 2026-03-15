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

  db_endpoint_no_proto = replace(scaleway_sdb_sql_database.todo.endpoint, "postgres://", "")
  db_host_part = strcontains(local.db_endpoint_no_proto, "@") ? split("@", local.db_endpoint_no_proto)[1] : local.db_endpoint_no_proto
  db_id = split(".", local.db_host_part)[0]

  db_url = "postgres://${scaleway_iam_application.db_app.id}:${scaleway_iam_api_key.db_key.secret_key}@${local.db_host_part}"

  db_url_with_options = "postgres://${scaleway_iam_application.db_app.id}:${scaleway_iam_api_key.db_key.secret_key}@${local.db_id}.pg.sdb.fr-par.scw.cloud:5432/${var.db_name}?sslmode=require&options=databaseid%3D${local.db_id}"
}

resource "null_resource" "docker_build_push" {
  for_each = { for f in local.functions : f.name => f }

  triggers = {
    dockerfile_hash = filemd5("${path.module}/../${each.value.dockerfile}")
    source_hash     = sha1(join("", [for f in fileset("${path.module}/../todo-api/src", "**") : filebase64sha256("${path.module}/../todo-api/src/${f}")]))
    cargo_hash      = filemd5("${path.module}/../todo-api/Cargo.toml")
  }

  provisioner "local-exec" {
    command = <<EOT
      docker build --platform linux/amd64 -t ${scaleway_registry_namespace.todo.endpoint}/${each.value.name}:latest -f ../${each.value.dockerfile} ..
      echo "${var.secret_key}" | docker login ${scaleway_registry_namespace.todo.endpoint} -u nologin --password-stdin
      docker push ${scaleway_registry_namespace.todo.endpoint}/${each.value.name}:latest
    EOT
  }
}

resource "null_resource" "db_init" {
  triggers = {
    schema_hash = filemd5("${path.module}/../infrastructure/postgres/init.sql")
    db_url      = local.db_url_with_options
  }

  provisioner "local-exec" {
    environment = {
      PGPASSWORD = scaleway_iam_api_key.db_key.secret_key
    }

    command = "sleep 90 && psql \"postgres://${scaleway_iam_application.db_app.id}@${local.db_id}.pg.sdb.fr-par.scw.cloud:5432/${var.db_name}?sslmode=require&options=databaseid%3D${local.db_id}\" -f ../infrastructure/postgres/init.sql"
  }

  depends_on = [scaleway_sdb_sql_database.todo, scaleway_iam_api_key.db_key]
}

resource "scaleway_container" "todo" {
  for_each     = { for f in local.functions : f.name => f }
  name         = "todo-${each.value.name}"
  namespace_id = scaleway_container_namespace.main.id
  registry_image = "${scaleway_registry_namespace.todo.endpoint}/${each.value.name}:latest"
  port         = 8080
  cpu_limit    = 100
  memory_limit = 128
  min_scale    = 0
  max_scale    = 3
  timeout      = 60
  deploy       = true

  environment_variables = {
    DATABASE_URL = local.db_url_with_options
  }

  depends_on = [null_resource.docker_build_push, null_resource.db_init]
}

resource "scaleway_sdb_sql_database" "todo" {
  name    = var.db_name
  max_cpu = 15
}

resource "scaleway_iam_application" "db_app" {
  name            = "db-application-todo"
  organization_id = var.organization_id
}

resource "scaleway_iam_api_key" "db_key" {
  application_id = scaleway_iam_application.db_app.id
  expires_at     = "2026-09-15T18:46:00Z" # 6 months from current date (2026-03-15)
}

resource "scaleway_iam_policy" "db_policy" {
  name            = "allow-db-access-todo"
  description     = "Grants access to the serverless SQL database"
  organization_id = var.organization_id

  rule {
    permission_set_names = ["ServerlessSQLDatabaseReadWrite"]
    project_ids          = [var.project_id]
  }

  application_id = scaleway_iam_application.db_app.id
}

resource "random_id" "bucket" {
  byte_length = 4
}

resource "scaleway_object_bucket" "todo_ui" {
  name = "todo-ui-${random_id.bucket.hex}"
  region = var.region
  force_destroy = true
}

resource "scaleway_object_bucket_website_configuration" "todo_ui" {
  bucket = scaleway_object_bucket.todo_ui.name
  region = var.region
  index_document {
    suffix = "index.html"
  }
}
resource "scaleway_object_bucket_acl" "todo_ui" {
  bucket = scaleway_object_bucket.todo_ui.name
  region = var.region
  acl    = "public-read"
}

resource "null_resource" "ui_build_upload" {
  triggers = {
    # Trigger on source code changes
    ui_hash = sha1(join("", [for f in fileset("${path.module}/../todo-ui/src", "**") : filebase64sha256("${path.module}/../todo-ui/src/${f}")]))
    # Trigger on function URL changes
    add_url      = scaleway_container.todo["add"].domain_name
    delete_url   = scaleway_container.todo["delete"].domain_name
    get_all_url  = scaleway_container.todo["get-all"].domain_name
    edit_url     = scaleway_container.todo["edit"].domain_name
  }

  provisioner "local-exec" {
    working_dir = "${path.module}/../todo-ui"
    environment = {
      URL_FAAS_ADD          = "https://${scaleway_container.todo["add"].domain_name}"
      URL_FAAS_DELETE       = "https://${scaleway_container.todo["delete"].domain_name}"
      URL_FAAS_GET_ALL      = "https://${scaleway_container.todo["get-all"].domain_name}"
      URL_FAAS_EDIT         = "https://${scaleway_container.todo["edit"].domain_name}"
      AWS_ACCESS_KEY_ID     = var.access_key
      AWS_SECRET_ACCESS_KEY = var.secret_key
      AWS_DEFAULT_REGION    = var.region
    }
    command = <<EOT
      trunk build --release
      # Using aws-cli or scw CLI to upload to S3 compatible bucket
      # Assuming scw or aws cli is available and configured
      aws s3 sync dist/ s3://${scaleway_object_bucket.todo_ui.name} --endpoint-url https://s3.${var.region}.scw.cloud --acl public-read
    EOT
  }

  depends_on = [scaleway_container.todo, scaleway_object_bucket.todo_ui]
}