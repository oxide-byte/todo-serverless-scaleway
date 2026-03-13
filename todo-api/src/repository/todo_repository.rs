use sqlx::PgPool;
use tracing::info;
use crate::models::{Error, Todo};

pub struct TodoRepository {
    pool: PgPool,
}

impl TodoRepository {

    pub fn new(pool: PgPool) -> Self {
        info!("Initializing PostgreSQL TodoRepository");
        TodoRepository { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, Error> {
        let todos = sqlx::query_as::<_, Todo>(
            r#"
            SELECT id, title, description, status, created, updated
            FROM todo.todo
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(todos)
    }

    pub async fn get_todo(&self, id: &str) -> Result<Option<Todo>, Error> {
        let uuid = uuid::Uuid::parse_str(id)?;
        let todo = sqlx::query_as::<_, Todo>(
            r#"
            SELECT id, title, description, status, created, updated
            FROM todo.todo
            WHERE id = $1
            "#
        )
        .bind(uuid)
        .fetch_optional(&self.pool)
        .await?;

        Ok(todo)
    }

    pub async fn insert_todo(&self, todo: Todo) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO todo.todo (id, owner, title, description, status, created, updated)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(todo.id)
        .bind("default_owner")
        .bind(todo.title)
        .bind(todo.description)
        .bind(todo.status)
        .bind(todo.created)
        .bind(todo.updated)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_todo(&self, todo: Todo) -> Result<(), Error> {
        sqlx::query(
            r#"
            UPDATE todo.todo
            SET title = $2, description = $3, status = $4, updated = $5
            WHERE id = $1
            "#
        )
        .bind(todo.id)
        .bind(todo.title)
        .bind(todo.description)
        .bind(todo.status)
        .bind(todo.updated)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_todo(&self, id: &str) -> Result<(), Error> {
        let uuid = uuid::Uuid::parse_str(id)?;
        sqlx::query(
            r#"
            DELETE FROM todo.todo
            WHERE id = $1
            "#
        )
        .bind(uuid)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use chrono::Utc;
    use crate::models::Todo;
    use crate::repository::todo_repository::TodoRepository;

    #[tokio::test]
    #[ignore]
    async fn pg_get_todo_list() {
        let pool = PgPool::connect("postgres://todo:password@localhost:5432/todo").await.unwrap();
        let todo_repository = TodoRepository::new(pool);
        let todos = todo_repository.get_all().await.unwrap();
        println!("{:?}", todos);
    }
}