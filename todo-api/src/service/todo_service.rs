use chrono::Utc;
use sqlx::PgPool;
use tracing::instrument;
use serde_json::{json, Value};
use crate::models::{EditTodo, Error, NewTodo, Todo};
use crate::repository::todo_repository::TodoRepository;

#[instrument()]
pub async fn get_todos(
    pool: PgPool,
) -> Result<Value, Error> {
    tracing::info!("get_todos");

    let todo_repo = TodoRepository::new(pool);

    match todo_repo.get_all().await {
        Ok(todos) => {
            Ok(json!(todos))
        }
        Err(e) => {
            tracing::error!("error: {:?}", e);
            Err(e)
        }
    }
}

#[instrument()]
pub async fn get_todo(
    pool: PgPool,
    id: String,
) -> Result<Value, Error> {
    tracing::info!("get_todo [{}]", id);

    let todo_repo = TodoRepository::new(pool);

    match todo_repo.get_todo(&id).await {
        Ok(todo) => {
            Ok(json!(todo))
        }
        Err(e) => {
            tracing::error!("error: {:?}", e);
            Err(e)
        }
    }
}

#[instrument()]
pub async fn add_todo(
    pool: PgPool,
    todo: NewTodo,
) -> Result<Value, Error> {
    tracing::info!("add_todo");

    let item = Todo::new(todo.title.clone(), todo.description.clone());

    let todo_repo = TodoRepository::new(pool);

    match todo_repo.insert_todo(item.clone()).await {
        Ok(_) => Ok(json!(item)),
        Err(e) => {
            tracing::error!("error: {:?}", e);
            Err(e)
        }
    }
}

#[instrument()]
pub async fn edit_todo(
    pool: PgPool,
    todo: EditTodo,
) -> Result<Value, Error> {
    tracing::info!("edit_todo");

    let item = todo;
    let todo_repo = TodoRepository::new(pool);

    let existing_todo = todo_repo.get_todo(&item.id.to_string()).await?;
    if existing_todo.is_none() {
        return Err(format!("Todo [{}] not found", &item.id).into());
    }

    let updated_todo = Todo {
        id: item.id,
        title: item.title,
        description: item.description,
        status: item.status,
        created: existing_todo.clone().unwrap().created,
        owner: existing_todo.clone().unwrap().owner,
        updated: Some(Utc::now())
    };

    match todo_repo.update_todo(updated_todo.clone()).await {
        Ok(_) => Ok(json!(updated_todo)),
        Err(e) => {
            tracing::error!("error: {:?}", e);
            Err(e)
        }
    }
}

#[instrument()]
pub async fn delete_todo(
    pool: PgPool,
    id: String,
) -> Result<Value, Error> {
    tracing::info!("delete_todo [{}]", id);

    let todo_repo = TodoRepository::new(pool);
    todo_repo.delete_todo(&id).await?;

    Ok(json!({ "id": id }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_to_list() {
        let json = "[
        {
            \"created\": \"2024-02-19T19:20:54.702Z\",
            \"description\": \"description\",
            \"id\": \"9e4f98b6-e332-478e-b3d5-6be74e5f97c7\",
            \"title\": \"title\",
            \"status\": \"Active\"
        }
        ]";

        let parsed: Vec<Todo> = serde_json::from_str(&json).unwrap();

        assert!(!parsed.is_empty());
        println!("{:?}", parsed);
    }
}